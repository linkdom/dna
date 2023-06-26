# This is my effort to get better at memorizing data structures and algorithms 
In here I keep notes, this implementation is going to be in Rust since I have done it
in typescript once and i figured that doing it in a different programming language will
be beneficial.

## Algorithms

### Big O Time Complexity

##### Log = Base 2 log

Big O is a way to categorize the algorithms time or memory requirements based on input.  
It is not meant to be an exact measurement. Why? Because it takes time to create memory etc.
It is meant to generalize the growth of an algorithm.

Important concepts:
* Growth is with respect to the input
* Constants are dropped
* Worst case is usually the way we measure

A neat trick when it comes to complexity is to count the number of for loops:
* 1 Loop = O(N)
* 2 Loops = O(N^2)
* 3 Loops = O(N^3)

##### O(n log n) 
You half the amount of space you need to search, but you still have to search the whole space once.  
Example: **Quicksort**  

#### O(log n) 
You half the amount of space you need to search, but you only need to look at one point.  
Example: **Binary Search Tree**

#### O(sqrt(N))
Basically this is O(n log n) but instad of halfing you take the sqrt of n.  
Example: **TwoCrystalBalls**

### Binary Search
The Binary Search list is an algorithm that needs a haystack (which is the array) and a needle (which is the thing you want to find).
It looks into the middle, if the value is bigger it continues with the bottom half, if not with the top half.
Then halfes that again and so on.

It is implemented as follows:

```
export default function bs_list(haystack: number[], needle: number): boolean {

    let lo = 0;
    let hi = haystack.length;
    do {
        const m = Math.floor(lo + (hi - lo) / 2);
        const v = haystack[m];

        if (v === needle) {
            return true;
        } else if (v > needle) {
            hi = m;
        } else {
            lo = m + 1;
        }
        
    } while(lo < hi)

    return false;

}
```

### Recursion

Recursion is a function that calls itself until it reaches an exit condition.  
The simplest example is the following: 
```

function foo(n: number): number {

    // Base Case
    if(n === 1) {
        return 1;
    }

    // Recurse
    return n + foo(n - 1);

}

```

There are three important properties that every function needs to know:
* return address (where shall the function return to)
* return value (which value shall it return)
* arguments (which arguments does it have)

A Recursion consists of these three steps:
* pre (you can do something before you recurse)
* recurse (actually does the calling of the function)
* post (we can do something else after recusing)


### Maze Solver
So what we are doing in the MazeSolver is that we walk in the four directions (up, right, down, left)
and we see if the path is valid. If so, we move again in the four directions and again and again until
we reach the end.  
We shall not configure all the base cases in the recursive steps,
instead it is way cleaner to define them beforehand so we can focus
on recursing.  

The Maze:  
```
const maze = [
    "xxxxxxxxxx x",
    "x        x x",
    "x        x x",
    "x xxxxxxxx x",
    "x          x",
    "x xxxxxxxxxx",
];

```

The Base Cases:  
```

    // If we run out of the map
    if (curr.x < 0 || curr.x >= maze[0].length ||
        curr.y < 0 || curr.y >= maze.length) {

        return false;
    }

    // If we run into a wall
    if (maze[curr.y][curr.x] === wall) {
        return false;
    }

    // If we reach the end
    if (curr.x === end.x && curr.y === end.y) {
        path.push(end);
        return true;
    }

    // If we have already seen it
    if (seen[curr.y][curr.x]) {
        return false;
    }


```

The 3 Recursion Steps:  
```
    // pre
    seen[curr.y][curr.x] = true;
    path.push(curr);

    //recurse
    for (let i = 0; i < dir.length; i++) {
        const [x, y] = dir[i];
        if (
            walk(maze, wall, {
                x: curr.x + x,
                y: curr.y + y,
            }, end, seen, path)
        ) {
            return true;
        }
    }

    //post
    path.pop();

    return false;

```

How we start recursing: 
```
export default function solve(maze: string[], wall: string, start: Point, end: Point): Point[] {
    const seen: boolean[][] = [];
    const path: Point[] = [];

    for (let i = 0; i < maze.length; i++) {
        seen.push(new Array(maze[0].length).fill(false));
    }

    walk(maze, wall, start, end, seen, path);

    return path;
}
```

### Quicksort

Quicksort is an algorithm that halfes the input, takes the midpoint and creates a left and a right side.  
Then does that again and again.
It has no consistent big O.
The big O lies anywhere between best case O(n log n) and worst case O(n^2).
It is a divide and conquer strategie which means we divide the thing we have and then conquer the problem from there.
Quicksort implements as follows:
```
function qs(arr: number[], lo: number, hi: number): void {
    if(lo >= hi) {
        return;
    }

    const pivotIdx = partition(arr, lo, hi);

    qs(arr, lo, pivotIdx - 1);
    qs(arr, pivotIdx + 1, hi);
}

function partition(arr: number[], lo: number, hi: number): number {

    const pivot = arr[hi];

    let idx = lo - 1;

    for(let i = lo; i < hi; ++i) {
        if (arr[i] <= pivot) {
            idx++;
            const tmp = arr[i];
            arr[i] = arr[idx];
            arr[idx] = tmp;
        }
    }

    idx++;
    arr[hi] = arr[idx];
    arr[idx] = pivot;

    return idx;
}

export default function quick_sort(arr: number[]): void {
    qs(arr, 0, arr.length - 1);
}
```


## Data Types

### Arrays

Arrays are 0(1) on everything.
All an array really is is a contiguous(unbreaking) memory space that contains a certain amount of bytes.
`[4Bytes, 4Bytes, 4Bytes, ...]`
4 Bytes === 32 Bits

Downside is that you need to create all the memory up front.

### Singly Linked List and Doubly Linked List

You need to only create the memory space that you will actually need.
Getting sucks on linked lists;
You need to walk the whole thing if you want to get an Item out of it
which is always O(N).  

It consits of 6 main functions:
* prepend(item: T): void
* insertAt(item: T, idx: number): void
* append(item: T): void
* remove(item: T): T | undefined
* get(idx: number): T | undefined
* removeAt(idx: number): T | undefined

Linked lists consist of 2 Properties:
* public length: number;
* private head?: Node<T>

where Node is defined as the following:
```
type Node<T> = {
    value: T,
    prev?: Node<T>, //ONLY ON DOUBLY LINKED LIST
    next?: Node<T>,
}
```
### Queue

The typical identifier of a Queue is that it is FIFO principle (First in, First Out) and
it consists of 3 main functions:
* enqueue(item: T): void
* deque(): T | undefined
* peek(): T | undefined

Queues consist of 3 Properties: 

* public length: number;
* private head?: Node<T>;
* private tail?: Node<T>;

where Node is defined as the following: 

```
type Node<T> = {
    value: T,
    next?: Node<T>,
}
```

### Stack

A Stack is the opposite of a Queue, it is defined by the LIFO principle (Last In, First Out) and
it consists of 3 main functions: 

* push(item: T): void
* pop(): T | undefined
* peek(): T | undefined

Stacks consist of 2 Properties:
* public length: number;
* private head?: Node<T>;

where Node is defined as the following:
```
type Node<T> = {
    value: T,
    prev?: Node<T>
}
```

### ArrayList
We want Array access with the ability to grow.
ArrayList's un/shift(or enqueue and deque) functions are O(n).  
We grow with always doubling the capacity.
Removing an item from the front sucks on array lists (because you need to shift everything one place to the left)

It consists of 6 main functions (they are the same as Linked Lists): 
* prepend(item: T): void
* insertAt(item: T, idx: number): void
* append(item: T): void
* remove(item: T): T | undefined
* get(idx: number): T | undefined
* removeAt(idx: number): T | undefined

## Side effect learnings

### Deep Copies

A deep copy is a copy of a data type that includes the copying of the memory location to a different place
which has the side effect of not manipulating the original memory location.

### Shallow Copies

Exactly the reverse of a deep copy. It just points to the memory location of the original and so if you try
to modify the copied version the original will also be modified.

### Passing functions in typescript

You can pass a function into another function which is defined as `function something(functionname: () => returntype): returntype`
The function is called with the new functionname plus braces.

```
function time(fn: () => void): number {
    const now = Date.now();
    fn();
    return Date.now() - now;
}
```

### Arrays

An array is always ungrowable. This means that the javascript "[]" can be identified as not being an array
because it is growable. The [] in javascript is actually an ArrayList!

### ArrayBuffer

In Javascript an ArrayBuffer is basically a traditional Array!
It's default type is a Uint8Content.

It is defined as the following:
```
const a = new ArrayBuffer(length);
const a8 = new Uint8Array(a);

// It is also possible to interpret it differently and reuse it like:
const a16 = new Uint16Array(a)

// Sample Output:
// ArrayBuffer { [Uint8Contents]: <00 00 00 00 00 00>, byteLength: 6 }
```
The thing that happens on the Uint16Array is that it will always take two 4Byte places for one index.

### Globals

Globals shouldn't really be used because they interfere with libraries etc, but if you only write a small package
that doesn't really have dependencies and you want to define global things, you can specify them in the
***globals.d.ts*** file

## Terminology

**root**: The most parent node. The first. Adam.  
**height**: The longest path from the root to the most child node  
**binary tree**: A tree that has at most 2 children, at least 0  
**general tree**: A tree with 0 or more children  
**binary search tree**: a tree in which has a specific ordering to the nodes and at most 2 children  
**leaves**: A node without children  
**balanced**: A tree is ***perfectly*** balanced when any node's left and right children have the same height.  
**branching factor**: The amount of children a tree has  
**traversal**: An attempt to visit every single node

## Things to learn

* Dynamic Programming
    * Stock Chart Problem
* Merge Sort
* Breadth-first search
* Depth first search
