from typing import Final
import sys
from dataclasses import dataclass
import math as m
import itertools
from pathlib import Path
import re


@dataclass
class Item:
    cost: int
    dmg: int
    armor: int


@dataclass
class Stats:
    hp: int
    dmg: int
    armor: int


PLAYER_INITIAL_HP: Final = 100

# dagger, shortsword, warhammer, longsword, greataxe
WEAPONS: Final = [
    Item(8, 4, 0),
    Item(10, 5, 0),
    Item(25, 6, 0),
    Item(40, 7, 0),
    Item(74, 8, 0),
]

# leather, chainmail, splintmail, badnedmail, platemail
ARMOR: Final = [
    Item(13, 0, 1),
    Item(31, 0, 2),
    Item(53, 0, 3),
    Item(75, 0, 4),
    Item(102, 0, 5),
]

# damage 1, damage 2, damage 3, defense 1, defense 2, defense 3
RINGS: Final = [
    Item(25, 1, 0),
    Item(50, 2, 0),
    Item(100, 3, 0),
    Item(20, 0, 1),
    Item(40, 0, 2),
    Item(80, 0, 3),
]


def is_win(player: Stats, boss: Stats) -> bool:
    player_attacks = m.ceil(boss.hp / max((player.dmg - boss.armor), 1))
    boss_atacks = m.ceil(player.hp / max((boss.dmg - player.armor), 1))
    return player_attacks <= boss_atacks  # player has first-move advantage


def find_min_cost(boss: Stats) -> int | None:
    min_cost = None
    armor_choices = ARMOR + [Item(0, 0, 0)]  # optional
    ring_choices: list[tuple[Item, ...]] = [()]
    ring_choices.extend(itertools.combinations(RINGS, 1))
    ring_choices.extend(itertools.combinations(RINGS, 2))
    for weapon, armor, rings in itertools.product(WEAPONS, armor_choices, ring_choices):
        items = (weapon, armor, *rings)
        player = Stats(
            PLAYER_INITIAL_HP,
            sum(map(lambda x: x.dmg, items)),
            sum(map(lambda x: x.armor, items)),
        )
        cost = sum(map(lambda x: x.cost, items))
        if is_win(player, boss) and (min_cost is None or cost < min_cost):
            min_cost = cost

    return min_cost

def find_max_cost(boss: Stats) -> int | None:
    max_cost = None
    armor_choices = ARMOR + [Item(0, 0, 0)]  # optional
    ring_choices: list[tuple[Item, ...]] = [()]
    ring_choices.extend(itertools.combinations(RINGS, 1))
    ring_choices.extend(itertools.combinations(RINGS, 2))
    for weapon, armor, rings in itertools.product(WEAPONS, armor_choices, ring_choices):
        items = (weapon, armor, *rings)
        player = Stats(
            PLAYER_INITIAL_HP,
            sum(map(lambda x: x.dmg, items)),
            sum(map(lambda x: x.armor, items)),
        )
        cost = sum(map(lambda x: x.cost, items))
        if not is_win(player, boss) and (max_cost is None or cost > max_cost):
            max_cost = cost

    return max_cost


def read_boss(path: Path) -> Stats:
    with open(path, "r") as f:
        file = f.read()

    boss = Stats(0, 0, 0)

    hp_matches = re.search(r"^Hit Points:\s+([0-9]+)$", file, re.MULTILINE)
    if hp_matches is None:
        raise RuntimeError("input did not contain hit points")
    boss.hp = int(hp_matches.group(1))
    damage_matches = re.search(r"^Damage:\s+([0-9]+)$", file, re.MULTILINE)
    if damage_matches is None:
        raise RuntimeError("input did not contain damage")
    boss.dmg = int(damage_matches.group(1))
    armor_matches = re.search(r"^Armor:\s+([0-9]+)$", file, re.MULTILINE)
    if armor_matches is None:
        raise RuntimeError("input did not contain armor")
    boss.armor = int(armor_matches.group(1))

    return boss


def main():
    if len(sys.argv) < 2:
        print("no input provided", file=sys.stderr)
        return
    file = sys.argv[1]
    try:
        boss = read_boss(Path(file))
    except Exception as e:
        print(f"failed to read `{file}`: {e}")
        return

    part1 = find_min_cost(boss)
    print(f"Part 1: {part1}")

    part2 = find_max_cost(boss)
    print(f"Part 2: {part2}")


if __name__ == "__main__":
    main()
