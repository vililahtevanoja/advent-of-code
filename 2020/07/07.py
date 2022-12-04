lines = []
with open('input.txt', 'r') as f:
    lines = f.readlines()
    
bag_dict = {}
for line in lines:
    words = line.split(' ')
    if words[4] == "no": continue
    d = { "name": "{} {}".format(words[0],words[1]), "bags": []}
    d["bags"].append({ "name": words[5] + " " + words[6], "count": int(words[4]) })
    remaining_bags = line.split(',')[1:]
    for bag in remaining_bags:
        words = [b for b in bag.split(' ') if b != ""]
        name = "{} {}".format(words[1].strip(), words[2].strip())
        count = int(words[0].strip())
        bag_info = {"name": name, "count": count}
        d["bags"].append(bag_info)
    bag_dict[d["name"]] = d["bags"]

def rec_bag_count(bag_dict, name, inner_bag_name):
    if name not in bag_dict:
        return 0
    bags = bag_dict[name]
    count = 0
    for b in bags:
        if b["name"] == inner_bag_name:
            count += 1
        else:
            count += rec_bag_count(bag_dict, b["name"], inner_bag_name)
    return count

shiny_gold_count = 0
for name, _ in bag_dict.items():
    if rec_bag_count(bag_dict, name, "shiny gold") > 0:
        shiny_gold_count += 1
print("Part 1: ", shiny_gold_count)
def rec_inner_bag_count(bag_dict, name):
    if name not in bag_dict:
        return 0
    bags = bag_dict[name]
    count = 0
    for bag in bags:
        count += bag["count"]
        count += bag["count"] * rec_inner_bag_count(bag_dict, bag["name"])
    return count
p2 = rec_inner_bag_count(bag_dict, "shiny gold")
print("Part 2: ", p2)
