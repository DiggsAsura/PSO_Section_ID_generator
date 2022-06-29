section_ids = ["Virdia", "Greenill", "Skyly", "Bluefull", "Purplenum", "Pinkal", "Redria", "Oran", "Yellowboz", "Withill"]
zer = ['F', 'P', 'Z', 'd', 'n', 'x', '2', '(', '<']
one = ['G', 'Q', 'e', 'o', 'y', '3', ')', '=', '[']
two = ['H', 'R', 'f', 'p', 'z', '4', '*', '\\', '>', ' ']
thr = ['I', 'S', 'g', 'q', '5', '!', '+', '{', ']', '?']
fou = ['J', 'T', 'h', 'r', '6', '@', '^', '}', '"', ',']
fiv = ['K', 'U', 'i', 's', '7', '#', '-', '_', '|']
six = ['B', 'L', 'V', 'j', 't', '8', '~', '$', '\'', '.']
sev = ['C', 'M', 'W', 'a', 'k', 'u', '9', '%', '/']
eig = ['D', 'N', 'X', 'b', 'l', 'v', '0', '&', ':']
nin = ['E', 'O', 'Y', 'c', 'm', 'w', '1', '`', ';']

print("\n\tWelcome to PSO Section ID Helper!")
print("\t===================================\n")
print("\tThis program will help you get your desired Section ID.")

print("\n\tAvailable IDs:\n")
for id in enumerate(section_ids):
    print(f'\t{id}')

print("")

# DESIRED SECTION ID
choice = input("\tDesired Section ID: ")

# DESIRED NAME
name = input("\tDesired character name: ")

# LOGIC 1: Calculate total value of name
score = 0
for c in name:
    if c in zer:
        score += 0
    elif c in one:
        score += 1
    elif c in two:
        score += 2
    elif c in thr:
        score += 3
    elif c in fou:
        score += 4
    elif c in fiv:
        score += 5
    elif c in six:
        score += 6
    elif c in sev:
        score += 7
    elif c in eig:
        score += 8
    elif c in nin:
        score += 9

# LAST DIGIT IN THE TOTAL NUMBER
last_dig = score % 10

# WHAT YOU WANT TO BE
msg1 = "You want to become"
match choice.strip():
    case 0:
        print(f'\t{msg1}: {section_ids[0]}')
    case 1:
        print(f'\t{msg1}: {section_ids[1]}')
    case 2:
        print(f'\t{msg1}: {section_ids[2]}')
    case 3:
        print(f'\t{msg1}: {section_ids[3]}')
    case 4:
        print(f'\t{msg1}: {section_ids[4]}')
    case 5:
        print(f'\t{msg1}: {section_ids[5]}')
    case 6:
        print(f'\t{msg1}: {section_ids[6]}')
    case 7:
        print(f'\t{msg1}: {section_ids[7]}')
    case 8:
        print(f'\t{msg1}: {section_ids[8]}')
    case 9:
        print(f'\t{msg1}: {section_ids[9]}')
    case _:
        print("\tNo match. Did you enter a number 0-9? Please try again")

# WHAT YOU BECAME
match last_dig:
    case 0:
        print(f'\tYou became {secion_ids[0]}')
    case 1:
        print(f'\tYou became {section_ids[1]}')
    case 2:
        print(f'\tYou became {section_ids[2]}')
    case 3:
        print(f'\tYou became {section_ids[3]}')
    case 4:
        print(f'\tYou became {section_ids[4]}')
    case 5:
        print(f'\tYou becamse {section_ids[5]}')
    case 6:
        print(f'\tYou became {section_ids[6]}')
    case 7:
        print(f'\tYou became {section_ids[7]}')
    case 8:
        print(f'\tYou became {section_ids[8]}')
    case 9:
        print(f'\tYou became {section_ids[9]}')



# WHAT YOU NEED TO ADD:


