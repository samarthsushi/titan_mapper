# mollweide mapper
math referenced from: https://en.wikipedia.org/wiki/Mollweide_projection#Mathematical_formulation

## main.rs line 36 consts
### all pixels that have an adjacent white pixel are stored in an array
the reason for this is that the map does not include any white pixels so we would only extract the boundary pixels.

### use floodfill to find clusters
there were some pixels that were being extracted from step one that were not part of the boundary (because of the white pixels inside the hollow spaces inside the label text inside map or some extremely light pixels)

the largest cluster would surely be the elliptical boundary

### find rightmost, leftmost, topmost, and bottommost points
from this cluster, find these points

### optimization
if we know the input map has a fully white background we don't need to performing clustering, since any anomalies would have to lie inside the ellipse (therefore, they cannot be any of the extreme points).

in step one, as you iterate over the points, just save the four extremes that satisfy the criteria

## screenshots
![sc1](./data/sc1.png)
![sc2](./data/sc1.png)
