

def get_last_output(input_subset, mapper_dict):
    """Performs a series of mappings on a given input subset using a dictionary of mappers.

    The order of mapping is the natural order of keys in the ``dict``.

    Args:
        input_subset (Subset): The initial subset to start the mapping process.
        mapper_dict (dict): A dictionary containing mappers to be applied sequentially.

    Returns:
        (Subset): The final subset obtained after applying all the mappers.
    """
    sorted_key_list = sorted(list(mapper_dict.keys()))
    intermediate_output = input_subset
    for key in sorted_key_list:
        intermediate_output = mapper_dict[key].map(intermediate_output)
    return intermediate_output

class Interval:
    def __init__(self, start, end):
        """Initializes an Interval object with a start and end value.

        Note that it's only right-closed, i.e. ``end`` is not included in the interval.

        Args:
            start (int): The starting value of the interval.
            end (int): The ending value of the interval (exclusive).
        """
        # end is not included
        self.start = start
        self.end = end
        self.width = self.end - self.start

    def contains(self, number: int):
        """Checks if the given number is contained within the interval.

        Args:
            number (int): The number to check.

        Returns:
            (bool): True if the number is within the interval, False otherwise.
        """
        return self.start <= number < self.end

    def intersection(self, other):
        """Finds the intersection of two intervals.

        Args:
            other (Interval): Another Interval object.

        Returns:
            (Interval) or None: The intersection interval if it exists, None otherwise.
        """
        if self.contains(
            other.start) or self.contains(
                other.end-1) or other.contains(
                    self.start) or other.contains(
                        self.end-1):
            return Interval(max(self.start, other.start), min(self.end, other.end))
    
    def unions(self, other):
        """Finds the union of two intervals.

        Note: tangent intervals (e.g. [1, 5) and [5, 8)) are considered disjoint and
        return a subset of two intervals.

        Args:
            other (Interval): Another Interval object.

        Returns:
            (Interval) or (Subset): The union interval if self intersects other, otherwise
                a Subset of two intervals.
        """
        if self.intersection(other):
            return Interval(min(self.start, other.start), max(self.end, other.end))
        else:
            return Subset([self, other])
        
    def difference(self, other):
        """Finds the difference between two intervals.

        Args:
            other (Interval): Another Interval object.

        Returns:
            (Interval) or None: The difference interval if it exists, None otherwise.
        """
        # no intersection
        if not self.intersection(other):
            return self
        # other contains self
        if other.contains(self.start) and other.contains(self.end-1):
            return None
        # difference on the left
        if self.start < other.start:
            return Interval(self.start, other.start)
        # difference on the right
        else:  # self.start >= other.start and self.end>other.start
            return Interval(other.end, self.end)
    
    def __repr__(self) -> str:
        return f"[{self.start}, {self.end})"
    def __eq__(self, other):
        return self.start==other.start and self.end==other.end
        
class Subset:
    # collection of intervals
    def __init__(self, interval_list):
        """Initializes a Subset object with a list of intervals.

        Args:
            interval_list (list): A list of Interval objects.
        """
        self.content = self._clean_intervals(interval_list)
    
    def _clean_intervals(self, intervals_list):
        """Cleans a list of intervals to make them mutually disconnected and ordered.

        Args:
            intervals_list (list): A list of Interval objects.

        Returns:
            (list): The cleaned list of mutually disconnected and ordered Interval objects.
        """
        if not intervals_list:
            return []
        sorted_intervals_list = sorted(intervals_list, key=lambda x: x.start)
        clean_intervals_list = [sorted_intervals_list.pop(0)]
        while sorted_intervals_list:
            interval = clean_intervals_list[-1]
            j = 0  # tracks other intervals
            for other_interval in sorted_intervals_list:
                if interval.intersection(other_interval):
                    # union is interval because there is intersection
                    interval = interval.unions(other_interval)
                    clean_intervals_list[-1] = interval
                else:
                    # new disconnected from old interval
                    clean_intervals_list.append(sorted_intervals_list[j])
                    break
                j += 1
            sorted_intervals_list = sorted_intervals_list[j+1:]  # empty if j+1>=len()
        return clean_intervals_list
    
    def __iter__(self):
        return iter(self.content)
    def __getitem__(self, index):
        return self.content[index]
    def __repr__(self) -> str:
        return str(self.content)


class Mapper:
    def __init__(self, domain, image, mapping_dict):
        """Initializes a Mapper object with domain, image, and a mapping dictionary.

        Note: If an element of the input don't have an image, it gets mapped to itself.

        Args:
            domain (Subset): The domain subset for the mapping.
            image (Subset): The image subset for the mapping.
            mapping_dict (dict): A dictionary mapping start of each interval in the 
                domain to the start of the interval's image.

        Note:
            The domain and image should be Subset objects.

        Example:
        ```
        domain = Subset([Interval(1, 5), Interval(8, 10)])
        image = Subset([Interval(100, 104), Interval(108, 110)])
        mapping_dict = {1: 100, 8: 108}
        mapper = Mapper(domain, image, mapping_dict)
        input_subset = Subset([Interval(3, 7)]
        print(mapper.map(input_subset)) # [[5, 7), [102, 104)]
        ```
        """
        self.domain = domain  # subset
        self.image = image  # images of intervals in domain
        self.ـmap = mapping_dict.copy()

    def map(self, subset):
        """Maps a subset using the Mapper.

        Note: If an element of the input don't have an image, it gets mapped to itself.

        Args:
            subset (Subset): The subset to be mapped.

        Returns:
            (Subset): The resulting subset after mapping.

        Example:
        ```
        domain = Subset([Interval(1, 5), Interval(8, 10)])
        image = Subset([Interval(100, 104), Interval(108, 110)])
        mapping_dict = {1: 100, 8: 108}
        mapper = Mapper(domain, image, mapping_dict)
        input_subset = Subset([Interval(3, 7)]
        print(mapper.map(input_subset)) # [[5, 7), [102, 104)]
        ```
        """
        all_image_list = []
        for input_interval in subset:
            total_width = input_interval.width  # to verify everything is mapped
            all_mapped = False
            image_list = []
            for domain_interval in self.domain:
                if domain_interval.intersection(input_interval):
                    input_intersection = domain_interval.intersection(input_interval)
                    input_start_dist = input_intersection.start - domain_interval.start
                    input_image_start = self.ـmap[domain_interval.start] + input_start_dist
                    input_image_width = input_intersection.width
                    image_list.append(
                        Interval(input_image_start, input_image_start+input_image_width))
                    if sum(interval.width for interval in image_list) == total_width:
                    # if domain_interval.contains(end-1):
                        all_mapped = True
                        break
                    else:
                        input_interval = input_interval.difference(domain_interval)

            if not all_mapped:
                image_list.append(Interval(input_interval.start, input_interval.end))
            all_image_list += image_list
        return Subset(all_image_list)
