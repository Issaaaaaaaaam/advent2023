# pyline: disable=consider-using-enumerate

def get_legal_directions(r, c):
    """look away man
    
    please perform an emergency backflip away from your 
    screen in order to not suffer mental retardation 
    from looking at this piece of code"""
    legal = directions.copy()
    if r == 0:
        legal.remove((-1, 0))
        legal.remove((-1, -1))
        legal.remove((-1, 1))
    elif r == 139: # 9 for demo, 139 for full
        legal.remove((1, 0))
        legal.remove((1, -1))
        legal.remove((1, 1))
    if c == 0:
        legal.remove((0, -1))
        try: # these may or may not have been removed in the row checks... right...
            legal.remove((-1, -1))
        except:
            pass
        try:
            legal.remove((1, -1))
        except:
            pass
    elif c == 139: # 9 for demo, 139 for full
        legal.remove((0, 1))
        try: # these may or may not have been removed in the row checks... right...
            legal.remove((-1, 1))
        except:
            pass
        try:
            legal.remove((1, 1))
        except:
            pass
    return legal

b = open("test_input.txt", "r").read().split("\n")
directions = [
    (-1, 0), # up
    (1, 0),  # down
    (0, -1), # left
    (0, 1),  # right
    (-1, -1),# up-left
    (-1, 1), # up-right
    (1, -1),  # down-left
    (1, 1),  # down-right
]

def p1():
    sum = 0
    part = False
    for r in range(len(b)):
        b[r] += "."
        num = ""
        for c in range(len(b[r])): 
            entry:str = b[r][c]
            if not entry.isdigit(): # if not a number move on
                if part:
                    sum += int(num)
                    part = False
                num = ""
                continue
            num += entry
            if part: # if already found that it is valid continue reading the rest of the number
                continue
            legal_directions = get_legal_directions(r, c)
            for d in legal_directions: #check 8 directions
                target = b[r+d[0]][c+d[1]]
                if target == "." or target.isdigit(): # if not special, continue looking
                    continue
                part = True # found valid part
    print(sum)

def p2():
    out = 0
    part = False
    gears = {}
    gear_memory = tuple()
    for r in range(len(b)):
        b[r] += "."
        num = ""
        for c in range(len(b[r])): 
            entry:str = b[r][c]
            if not entry.isdigit(): # if not a number move on
                if part:
                    if gear_memory:
                        gears[gear_memory].append(int(num))
                        gear_memory = tuple()
                    part = False
                num = ""
                continue
            num += entry
            if part: # if already found that it is valid continue reading the rest of the number
                continue
            legal_directions = get_legal_directions(r, c)
            for d in legal_directions: #check 8 directions
                target = b[r+d[0]][c+d[1]]
                if target == "." or target.isdigit(): # if not special, continue looking
                    continue
                part = True # found valid part
                if target == "*":
                    gear_memory = (r+d[0],c+d[1])
                    if gear_memory not in gears:
                        gears[gear_memory] = []
    
    out = sum([value[0] * value[1] for value in gears.values() if len(value) == 2])
    print(out)


p2()