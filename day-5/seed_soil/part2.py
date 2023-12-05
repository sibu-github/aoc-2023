# [Note]: Copied some code from internet
# Need to rewrite this logic in Rust
input = open("input.txt").read().strip()
seeds, *blocks = input.split('\n\n')
seeds = list(map(int, seeds.split(': ')[1].split()))

seeds_ranges = []
for i in range(0, len(seeds), 2):
    seeds_ranges.append((seeds[i], seeds[i] + seeds[i+1]))

for block in blocks:
    mapping_ranges = []
    for line in block.split('\n')[1:]:
        mapping_ranges.append(list(map(int, line.split())))
    
    mapped_seed_ranges = []

    while seeds_ranges:
        seed_start, seed_end = seeds_ranges.pop()
        range_mapped = False

        for dst, src, sz in mapping_ranges:
            # split the ranges
            ovlp_start = max(seed_start, src)
            ovlp_end = min(seed_end, src+sz)

            if ovlp_start < ovlp_end:
                mapped_seed_ranges.append((ovlp_start - src + dst, ovlp_end - src + dst))

                if seed_start < ovlp_start:
                    seeds_ranges.append((seed_start, ovlp_start))
                if ovlp_end < seed_end:
                    seeds_ranges.append((ovlp_end, seed_end))
                range_mapped = True
                break

        if not range_mapped:
            mapped_seed_ranges.append((seed_start, seed_end))

    seeds_ranges = mapped_seed_ranges

print(min(seeds_ranges)[0])