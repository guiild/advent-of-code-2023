import pytest
import numpy as np

from utils import Interval, Subset, Mapper

@pytest.fixture
def sample_intervals():
    interval1 = Interval(1, 5)
    interval2 = Interval(3, 7)
    interval3 = Interval(6, 8)
    interval4 = Interval(8, 10)
    interval5 = Interval(12, 15)
    interval6 = Interval(1, 15)
    return interval1, interval2, interval3, interval4, interval5, interval6

def test_contains():
    interval = Interval(1, 5)
    assert np.all([interval.contains(i) for i in range(1, 5)])
    assert not interval.contains(5)
    assert not interval.contains(0)

    interval = Interval(1, 15)
    assert interval.contains(6)
    assert interval.contains(7)

def test_intersection(sample_intervals):
    interval1, interval2, interval3, interval4, interval5, interval6 = sample_intervals
    intersection = interval1.intersection(interval2)
    assert intersection.start == 3
    assert intersection.end == 5

    no_intersection = interval1.intersection(interval3)
    assert no_intersection is None

    # tangent case
    no_intersection = interval3.intersection(interval4)
    assert no_intersection is None
    no_intersection = interval4.intersection(interval3)
    assert no_intersection is None

    # contain case 
    contain_intersection = interval1.intersection(interval6)
    assert isinstance(contain_intersection, Interval)
    assert contain_intersection == interval1
    contain_intersection = interval6.intersection(interval1)
    assert isinstance(contain_intersection, Interval)
    assert contain_intersection == interval1
    # another contain case 
    contain_intersection = interval3.intersection(interval6)
    assert isinstance(contain_intersection, Interval)
    assert contain_intersection == interval3


def test_unions(sample_intervals):
    interval1, interval2, interval3, interval4, interval5, interval6 = sample_intervals
    # intersect case
    union = interval1.unions(interval2)
    assert isinstance(union, Interval)
    assert union.start == 1
    assert union.end == 7

    # tangent case : should give two disjoint intervals
    tangent_union = interval3.unions(interval4)
    assert isinstance(tangent_union, Subset)
    assert len(tangent_union.content) == 2
    assert tangent_union[0].start == 6
    assert tangent_union[0].end == 8
    assert tangent_union[1].start == 8
    assert tangent_union[1].end == 10

    # no intersect case
    no_intersection_union = interval1.unions(interval5)
    assert isinstance(no_intersection_union, Subset)
    assert len(no_intersection_union.content) == 2
    assert no_intersection_union[0].start == 1
    assert no_intersection_union[0].end == 5
    assert no_intersection_union[1].start == 12
    assert no_intersection_union[1].end == 15

    # one contain the other
    contain_union = interval1.unions(interval6)
    assert isinstance(contain_union, Interval)
    assert contain_union.start == 1
    assert contain_union.end == 15
    contain_union = interval6.unions(interval1)
    assert isinstance(contain_union, Interval)
    assert contain_union.start == 1
    assert contain_union.end == 15

def test_difference(sample_intervals):
    interval1, interval2, interval3, interval4, interval5, interval6 = sample_intervals
    # far left: self is to the far left of other
    far_left_diff = interval1.difference(interval4)
    assert isinstance(far_left_diff, Interval)
    assert far_left_diff.start == 1
    assert far_left_diff.end == 5

    # left (tangent and intersect)
    ## tangent
    left_diff = interval3.difference(interval4)
    assert isinstance(left_diff, Interval)
    assert left_diff.start == 6
    assert left_diff.end == 8
    ## intersect
    left_diff = interval2.difference(interval3)
    assert isinstance(left_diff, Interval)
    assert left_diff.start == 3
    assert left_diff.end == 6

    # contain
    contain_diff = interval3.difference(interval6)
    assert contain_diff is None

    # right (tangent and intersect)
    right_diff = interval3.difference(interval2)
    assert isinstance(right_diff, Interval)
    assert right_diff.start == 7
    assert right_diff.end == 8

    # far right
    far_right = interval5.difference(interval2)
    assert isinstance(far_right, Interval)
    assert far_right.start == 12
    assert far_right.end == 15

    # identical
    identical_interval = interval1.difference(interval1)
    assert identical_interval is None

def test_clean_intervals(sample_intervals):
    interval1, interval2, interval3, interval4, interval5, interval6 = sample_intervals

    # case 1: empty list
    cleaned_empty_subset = Subset([])._clean_intervals([])
    assert len(cleaned_empty_subset) == 0

    # case 2: one single interval
    cleaned_single_interval_subset = Subset([])._clean_intervals([interval1])
    assert len(cleaned_single_interval_subset) == 1
    assert cleaned_single_interval_subset[0] == interval1

    # case 3: different intersection, tangent
    interval_list = [interval1, interval2, interval3, interval4, interval5]
    cleaned_subset = Subset([])._clean_intervals(interval_list)
    assert len(cleaned_subset) == 3
    assert cleaned_subset[0].start == 1
    assert cleaned_subset[0].end == 8
    assert cleaned_subset[1].start == 8
    assert cleaned_subset[1].end == 10
    assert cleaned_subset[2].start == 12
    assert cleaned_subset[2].end == 15

    # case 4: one interval contain everything
    interval_list.append(interval6)
    cleaned_subset = Subset([])._clean_intervals(interval_list)
    assert len(cleaned_subset) == 1
    assert cleaned_subset[0].start == 1
    assert cleaned_subset[0].end == 15


def test_map():
    domain = Subset([Interval(1, 5), Interval(8, 10)])
    image = Subset([Interval(100, 104), Interval(108, 110)])
    mapping_dict = {1: 100, 8: 108}
    mapper = Mapper(domain, image, mapping_dict)

    # Test mapping with full intersections between input and domain intervals
    input_subset1 = Subset([Interval(1, 5), Interval(8, 10)])
    result1 = mapper.map(input_subset1)
    assert len(result1.content) == 2
    assert result1[0] == Interval(100, 104)
    assert result1[1] == Interval(108, 110)

    # Test mapping with no intersections between input and domain intervals
    input_subset2 = Subset([Interval(15, 20), Interval(20, 30)])
    result2 = mapper.map(input_subset2)
    assert len(result2.content) == 2
    assert result2[0] == Interval(15, 20)
    assert result2[1] == Interval(20, 30)

    # Test mapping with partial intersections
    input_subset3 = Subset([Interval(3, 7), Interval(5, 8)])
    result3 = mapper.map(input_subset3)
    assert len(result3.content) == 2
    assert result3[0] == Interval(5, 8)
    assert result3[1] == Interval(102, 104)

    # Test mapping with differences
    input_subset4 = Subset([Interval(1, 10)])
    result4 = mapper.map(input_subset4)
    assert len(result4.content) == 3
    assert result4[0] == Interval(5, 8)
    assert result4[1] == Interval(100, 104)
    assert result4[2] == Interval(108, 110)
