with open("Input.txt", "r") as f:
    rules = f.readlines()

# next_iter = []
# count = 0


# def main(target_bag):
#     pass
["shiny gold"]


def part_1(target_bags):
    already_checked = []
    count = 0
    while len(target_bags) > 0:
        for target_bag in target_bags:
            for rule in rules:
                first_bag = rule.split(" bags")[0]
                if (rule.find(target_bag) != -1 and first_bag != target_bag and first_bag not in already_checked):
                    target_bags.append(first_bag)
                    already_checked.append(first_bag)
                    count += 1

            target_bags.remove(target_bag)

    return count


print(part_1(["shiny gold"]))

# global count
#     for rule in rules:
#         first_bag = rule.split(" bags")[0]
#         if (rule.find(target_bag) != -1 and first_bag != target_bag):
#             count += 1
#             next_iter.append(first_bag)

#     if (len(next_iter) > 0):
#         for target in next_iter:
#             next_iter.remove(target)
#             part_1(target)
#     else:
#         print(count)
