min, max = tuple(map(lambda x:int(x), input().split()))
# print(min, max)
# che = [1 for _ in range(max-min)]
p_list = [2,3,5,7,11,13,17,19,23,29]
for i in range(31,round(max**0.5),2):
    f = True
    for p in p_list:
        if not i%p:
            f = False
            break
    if f: p_list.append(i)

print(p_list)

# for p in p_list:
#     P = (p*p)
#     for i in range((min//P+1)*P, max, P):
#         che[i-min] = 0

# print(sum(che))