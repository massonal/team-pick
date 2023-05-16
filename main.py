
import json
import os
import random
from pprint import pprint


def load_team()->dict[str, int]:
    """
    Loads the content of stakes.json (in the same directory as __file__) and returns the result.
    """
    with open("stakes.json", "r") as file:
        team = json.load(file)
    return team


def store_team(team:dict[str, int])->None:
    """
    Stores the content of team in stakes.json.
    """
    with open("stakes.json", "w") as file:
        json.dump(team, file, sort_keys=True, indent=4)

def choose_chairman(team:dict[str, int])->str:
    """
    Chooses at random a chairman from the keys of team and returns it.
    The odds of being selected are the value associated with the key.
    """
    # Python dicts are guaranteed to be iterated in insertion order
    # so pepoles[i] always matches with weights[i]
    people = tuple(team.keys())
    weights = tuple(team.values())
    return random.choices(people, weights=weights)[0]


def update_stakes(team:dict[str, int], chairman:str)->None:
    """
    Sets the stakes of the chairman to 0 (so that no one can be chairman twice in a row).
    The stakes of the other team members are incremented.
    """
    for person in team:
        team[person] += 1
    team[chairman] = 0

if __name__ == '__main__':
    # change the current working directory to the directory of this file
    # to avoid scattering files...
    os.chdir(os.path.dirname(__file__))

    team = load_team()
    print("Stakes:")
    pprint(team)

    chairman = choose_chairman(team)
    print("\n\nChairman:", chairman, "\n\n")
    
    update_stakes(team, chairman)
    print("New stakes:")
    pprint(team)
    store_team(team)