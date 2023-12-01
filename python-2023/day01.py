def part_1() -> None:
    data_file = open('../data/2023/01.txt', 'r')
    lines = data_file.readlines()
    data_file.close()
    
    line_sum: int = 0
    for line in lines:
        number_string: str = ""
        for index in range(0, len(line)):
            if line[index].isnumeric():
                number_string += line[index]
        line_sum += int(number_string[0] + number_string[-1])
        
    print(line_sum)

def replace_written_digits(line: str) -> str:
    return line.replace("one", "1").replace("two", "2").replace("three", "3").replace("four", "4").replace("five", "5").replace("six", "6").replace("seven", "7").replace("eight", "8").replace("nine", "9")


replacements: list[tuple[str, int]] = [("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)]

def firstDigit(line: str) -> int:
    head = line[0]
    if head.isdigit():
        return int(head)
    
    for option in replacements:
        if line.startswith(option[0]):
            return option[1]
        
    rest = line[1:]
    return firstDigit(rest)

def lastDigit(line: str) -> int:
    last_char = line[-1]
    if last_char.isdigit():
        return int(last_char)
    
    for option in replacements:
        if line.endswith(option[0]):
            return option[1]
        
    rest = line[:-1]
    return lastDigit(rest)



def part_2() -> None:
    data_file = open('../data/2023/01.txt', 'r')
    lines = data_file.readlines()
    data_file.close()
    
    line_sum: int = 0
    for line in lines:
        first = firstDigit(line.strip())
        last = lastDigit(line.strip())
        line_sum += first * 10 + last
        
    print(line_sum)

part_1()
part_2()