# Building a 3D-Printing slicer in Rust : Detailed introduction

to-do write an intro

## How does a 3D-printer work ? 

​	I don't have the pretension to explain it perfectly. Add to that the fact that we will focus on a specific technology of 3D printers : FDM, which means _"Fused deposition modeling"_. It's the most affordable and simple 3D printing technology, and it's also the technology of my 3D printer.

​	To fabricate your ideas, an FDM 3D printer will deposit fused plastic, layer by layer, to slowly build volume. Each layer has the shape of an horizontal cross-section of the model, and is drawn by the extruder, line by line, like a kid coloring a drawing with a tiny pencil.

​	The thinner the layer, the highest the quality of the print will be. But the longer it will take to print. A typical FDM printer can print layers as thick as 0.2 mm and as thin as 0.01 mm.

​	This parameter and a lot more are important for a good print and a slicer should be able to compose with all of them. I will detail them later, I don't want to throw a bunch of numbers at your face !

## What is a slicer ?

Again, I will only explain the concepts we need to know for the project. The slicer it the piece of software that makes 3D printing possible. It takes a 3D model and transforms it into something the printer understand :

​	      3D model

​			 |

​		   Slicer

​			 |

​		.gcode file

The .gcode file is what the printer needs. It contains instructions that control the movement of the extruder, temperature, fans, extrusion speed, movement speed, etc.. The printer is actually very stupid. We can see the .gcode file as a compiled version of the 3D model and the slicer as a compiler.

A slicer implements features that are considered mandatory and basic but that are actually very complex :

	- Infill
	- Supports
	- Adhesion surfaces
	- etc...

We won't implement any of these features for the moment, maybe later. But it's good to know they exist.

A simple slicer can be describe by the following schema :

​	  parsing 3D model

​			 |

   Process cross sections

​			 |

 Process extruder movements

​			 |

​	 write .gcode file

It may seem obvious but let me state this : for a layer height of 0.2 mm, a cross-section is processed every 0.2 mm. I warned you !

Then, from this cross-section _(that we can definitely call a "slice")_ is derived a series of movement the head will have to perform : First it will describe "walls" corresponding of the edges of the shape and then, eventually, fill the interior of the slice.

## A naive approach

If I explained well, you maybe already have an idea :

- Compute cross-sections of the model every `H`, where `H` is the layer-height
- Draw the edges of the resulting polygon !

And `*boom*`, we have a very basic but functional slicer !

Let's try to implement it in pseudo rust so we can see the problems it raises :

```rust
struct Vertex {
	pub x: f32,
	pub y: f32,
	pub z: f32,    
}

type Segment = (Vertex, Vertex);
type Polygon = Vec<Segment>;
type Triangle = [Vertex; 3];

fn find_lowest(obj: Object) -> f32 {...}
fn find_highest(obj: Object) -> f32 {...}

fn is_intersected(seg: Segment, h: f32) -> bool {...}

fn get_triangle_intersection(tri: Triangle, h: f32) -> Segment {...}

fn gcode(slices: Vec<Polygon>) {...}

fn main() {
    let args = env::args.collect();
    let obj = parse(args[1]);
    
    let lowest_vertice = find_lowest(obj);
    let highest_vertice = find_highest(obj);
    
    const layer_height = 0.2;
    
    let mut slices: Vec<Polygon> = vec![];
    
    for h in range(lowest_vertice, highest_vertice, layer_height) {
        let mut slice: Polygon = vec![];
        
    	for triangle in obj.iter() {
            // We use labels to be sure what loop will break.
            'inner: for segment in triangle.iter() {
                if is_intersected(segment, h) {
                    let intersection = get_triangle_intersection(triangle, h);
                
                    slice.push(intersection);
                    break 'inner;
                }
        	}
    	}
        
        slices.push(slice);
    }
    
    gcode(slices);
}
```



## Problems

If we analyze the complexity of this pseudo program we have :

- for each slice : $H$
- for each triangle : $T$
- for each segment : $3$

$$
O(H * T * 3) \rightarrow O(H * T)
$$

Which is bad ! I refuse to simplify it to $O(n^2)$ because $H$ is usually orders of magnitude smaller than $T$. We can say it's pseudo $O(n^2)$ if you wish...

Anyway,

Me  :	There's a way to entirely remove $T$.

You :	WHAT ?

Me  :	Yes.

Removing $T$ means we already know what polygons to integrate into our fresh slice of 3D object. The obvious solution would be to sort our Triangles from top to bottom, but it won't be a big improvement. [explain]

## Stages

While discussing about this problem with a coworker, we found an elegant solution. By dividing the model in carefully chosen stages, we can avoid searching for polygons at every slice.

 
