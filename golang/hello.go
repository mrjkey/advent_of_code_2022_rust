package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	// fmt.Println(quote.Glass())
	dat, err := os.ReadFile("../inputs/files.txt")
	check(err)
	file_string := strings.Split(string(dat), "\n")
	root := create_file_tree(file_string)
	// for _, child := range root.children {
	// 	if child.is_file {
	// 		fmt.Println(child.name)
	// 	}
	// }
	calculate_directory_size(root)
	size := sum_directory_size(root)
	fmt.Println(size)
	total_space := 70000000
	used_space := root.size
	needed_unused_space := 30000000
	minimum_deletion_size := used_space - (total_space - needed_unused_space)
	deletion_size := get_deletion_directory_size(root, minimum_deletion_size)
	res, _ := fmt.Printf("needed %d, got %d", minimum_deletion_size, deletion_size)
	fmt.Println(res)
}

type Node struct {
	name     string
	is_file  bool
	size     int
	children []*Node
	parent   *Node
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func create_file_tree(file_string []string) *Node {
	root := Node{
		name:     "/",
		is_file:  false,
		size:     0,
		children: []*Node{},
		parent:   nil,
	}
	active_dir := &root
	for _, s := range file_string {
		if s != "" {
			fields := strings.Fields(s)
			if string(s[0]) == "$" {
				// command
				if len(fields) == 3 {
					// change dir command
					active_dir = change_dir(&root, active_dir, fields[2])
					// fmt.Println(active_dir.name)
				} else {
					// ls command
				}
			} else if string(s[0:3]) == "dir" {
				// directory
				name := fields[1]
				create_child(active_dir, name, 0, false)
			} else {
				// file
				size, _ := strconv.Atoi(fields[0])
				name := fields[1]
				create_child(active_dir, name, size, true)
			}
		}
	}
	return &root
}

func change_dir(root *Node, active_dir *Node, next_dir string) *Node {
	if next_dir == "/" {
		// fmt.Println("Change to root")
		return root
	} else if next_dir == ".." {
		if active_dir.name != "/" {
			return active_dir.parent
		} else {
			return root
		}
	} else {
		for _, child := range active_dir.children {
			if next_dir == child.name {
				return child
			}
		}
		fmt.Println("This child does not exist!")
	}
	return root
}

func create_child(active_dir *Node, name string, size int, is_file bool) {
	exists := false

	for _, child := range active_dir.children {
		if name == child.name {
			exists = true
		}
	}
	if !exists {
		active_dir.children = append(active_dir.children, &Node{
			name:     name,
			is_file:  is_file,
			size:     size,
			children: []*Node{},
			parent:   active_dir,
		})
	}
}

func calculate_directory_size(root *Node) int {
	for _, child := range root.children {
		if child.is_file {
			root.size += child.size
		} else {
			root.size += calculate_directory_size(child)
		}
	}
	return root.size
}

func sum_directory_size(root *Node) int {
	size := 0
	for _, child := range root.children {
		if !child.is_file {
			size += sum_directory_size(child)
		}
	}
	if root.size <= 100000 {
		size += root.size
	}
	return size
}

func get_deletion_directory_size(root *Node, minimum_deletion_size int) int {
	deletion_size := 70000000
	for _, child := range root.children {
		if !child.is_file {
			child_deletion_size := get_deletion_directory_size(child, minimum_deletion_size)
			if child_deletion_size >= minimum_deletion_size {
				if child_deletion_size < deletion_size {
					deletion_size = child_deletion_size
				}
			}
		}
	}
	if deletion_size > root.size {
		deletion_size = root.size
	}
	return deletion_size
}
