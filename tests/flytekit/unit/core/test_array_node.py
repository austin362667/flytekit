import typing
from collections import OrderedDict

import pytest

from flytekit import LaunchPlan, current_context, task, workflow
from flytekit.configuration import Image, ImageConfig, SerializationSettings
from flytekit.core.array_node import array_node
from flytekit.core.array_node_map_task import map_task
from flytekit.models.core import identifier as identifier_models
from flytekit.tools.translator import get_serializable


@pytest.fixture
def serialization_settings():
    default_img = Image(name="default", fqn="test", tag="tag")
    return SerializationSettings(
        project="project",
        domain="domain",
        version="version",
        env=None,
        image_config=ImageConfig(default_image=default_img, images=[default_img]),
    )


@task
def multiply(val: int, val1: typing.Union[int, str], val2: int) -> int:
    if type(val1) is str:
        return val * val2
    return val * int(val1) * val2


@workflow
def parent_wf(a: int, b: typing.Union[int, str], c: int = 2) -> int:
    return multiply(val=a, val1=b, val2=c)


lp = LaunchPlan.get_default_launch_plan(current_context(), parent_wf)


@workflow
def grandparent_wf() -> typing.List[int]:
    return array_node(lp, concurrency=10, min_success_ratio=0.9)(a=[1, 3, 5], b=["two", 4, "six"], c=[7, 8, 9])


def test_lp_serialization(serialization_settings):
    wf_spec = get_serializable(OrderedDict(), serialization_settings, grandparent_wf)
    assert len(wf_spec.template.nodes) == 1

    top_level = wf_spec.template.nodes[0]
    assert top_level.inputs[0].var == "a"
    assert len(top_level.inputs[0].binding.collection.bindings) == 3
    for binding in top_level.inputs[0].binding.collection.bindings:
        assert binding.scalar.primitive.integer is not None
    assert top_level.inputs[1].var == "b"
    for binding in top_level.inputs[1].binding.collection.bindings:
        assert binding.scalar.union is not None
    assert len(top_level.inputs[1].binding.collection.bindings) == 3
    assert top_level.inputs[2].var == "c"
    assert len(top_level.inputs[2].binding.collection.bindings) == 3
    for binding in top_level.inputs[2].binding.collection.bindings:
        assert binding.scalar.primitive.integer is not None

    serialized_array_node = top_level.array_node
    assert (
            serialized_array_node.node.workflow_node.launchplan_ref.resource_type
            == identifier_models.ResourceType.LAUNCH_PLAN
    )
    assert (
            serialized_array_node.node.workflow_node.launchplan_ref.name
            == "tests.flytekit.unit.core.test_array_node.parent_wf"
    )
    assert serialized_array_node._min_success_ratio == 0.9
    assert serialized_array_node._parallelism == 10

    subnode = serialized_array_node.node
    assert subnode.inputs == top_level.inputs


@pytest.mark.parametrize(
    "min_successes, min_success_ratio, should_raise_error",
    [
        (None, None, True),
        (None, 1, True),
        (None, 0.75, False),
        (None, 0.5, False),
        (1, None, False),
        (3, None, False),
        (4, None, True),
        # Test min_successes takes precedence over min_success_ratio
        (1, 1.0, False),
        (4, 0.1, True),
    ],
)
def test_local_exec_lp_min_successes(min_successes, min_success_ratio, should_raise_error):
    @task
    def ex_task(val: int) -> int:
        if val == 1:
            raise Exception("Test")
        return val

    @workflow
    def ex_wf(val: int) -> int:
        return ex_task(val=val)

    ex_lp = LaunchPlan.get_default_launch_plan(current_context(), ex_wf)

    @workflow
    def grandparent_ex_wf() -> typing.List[typing.Optional[int]]:
        return array_node(ex_lp, min_successes=min_successes, min_success_ratio=min_success_ratio)(val=[1, 2, 3, 4])

    if should_raise_error:
        with pytest.raises(Exception):
            grandparent_ex_wf()
    else:
        assert grandparent_ex_wf() == [None, 2, 3, 4]


def test_map_task_wrapper():
    mapped_task = map_task(multiply)(val=[1, 3, 5], val1=[2, 4, 6], val2=[7, 8, 9])
    assert mapped_task == [14, 96, 270]

    mapped_lp = map_task(lp)(a=[1, 3, 5], b=[2, 4, 6], c=[7, 8, 9])
    assert mapped_lp == [14, 96, 270]
