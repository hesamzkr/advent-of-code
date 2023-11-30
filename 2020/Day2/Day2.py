with open("Input.txt", "r") as f:
    lines = f.readlines()

def part_1():
    total_correct = 0
    for line in lines:
        args = line.split()

        lower_bound = int(args[0].split("-")[0])
        higher_bound = int(args[0].split("-")[1])

        check_for = args[1][0]

        password = args[2]

        count = 0
        for letter in password:
            if letter == check_for:
                count += 1

        if count >= lower_bound and count <= higher_bound:
            total_correct += 1
    
    return total_correct


def part_2():
    total_correct = 0
    for line in lines:
        args = line.split()

        positions = args[0].split("-")
        check_for = args[1][0]

        password = args[2]

        if int(positions[0]) - 1 <= len(password):
            first_check = True if password[int(positions[0]) - 1] == check_for else False
        else:
            second_check = False

        if int(positions[1]) - 1 <= len(password):
            second_check = True if password[int(positions[1]) - 1] == check_for else False
        else:
            second_check = False

        if first_check ^ second_check:
            total_correct += 1

    return total_correct


print(f"Part1 correct passwords: {part_1()}")
print(f"Part2 correct passwords: {part_2()}")