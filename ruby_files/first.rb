def get_move(string)
  myArray = string.split
  moves = Integer(myArray[1])
  start = Integer(myArray[3])-1
  final = Integer(myArray[5])-1
  return moves, start, final
end

def get_top_index(container_array, container_array_depth, column)
  top_index = 0
  (0..container_array_depth-1).each do |i|
    if container_array[i][column] != " "
      top_index = i
      break
    end
  end
  return top_index
end
  
if __FILE__ == $0
  content = File.read("inputs/stacks.txt")
  containers_collected = false;
  container_array = []
  init_array = false;
  container_array_depth = 0
  columns = 0
  content.each_line do |string|
    if containers_collected
      if not init_array
        (0..columns-1).each do |n|
          container_array[n] = container_array[n].reverse()
        end
        init_array = true
      end

      moves, start, final = get_move(string)
      # puts final
      
      # solution 1, uncomment to use it
      # (0..moves-1).each do
      #   container_array[final].push(container_array[start].pop())
      # end
      temp_array = []
      (0..moves-1).each do
        temp_array.push(container_array[start].pop())
      end
      (0..moves-1).each do
        container_array[final].push(temp_array.pop())
      end

      # temp_array = temp_array.reverse()


    elsif string == "" || string == "\n"
      # puts "ahhhh"
      containers_collected = true
      init_array = false
    elsif not containers_collected
      if not init_array
        (0..string.length-1).step(4) do |n|
          container_array.push([])
          columns += 1
        end
        init_array = true
      end
      col_index = 0
      (0..string.length-1).step(4) do |n|
        sub_string = string[n+1]
        if sub_string == "1"
          break
        end
        if sub_string != " "
          container_array[col_index].push(sub_string)
        end
        col_index += 1
      end
    end
  end
  final_string = ""
  (0..columns-1).each do |n|
    final_string.concat(container_array[n].last)
  end
  puts final_string
  # puts container_array[0]
end

