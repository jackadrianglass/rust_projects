class Node
  attr_accessor :next
  attr_accessor :val

  def initialize(value)
    @next = nil
    @val = value
  end
end

class LinkedList
  attr :head
  attr :size

  def initialize(values)
    @head = nil
    @size = 0
    for val in values.reverse do
      t = Node.new(val)
      if head == nil then
        @head = t
      else
        t.next = @head
        @head = t
      end
      @size += 1
    end
  end

  def push(val)
    if @head == nil then
      @head = Node.new(val)
      return
    end

    n = @head
    while n.next != nil do
      n = n.next
    end
    n.next = Node.new(val)
    @size += 1
  end

  def pop
    if @head == nil then
      return nil
    elsif @head.next == nil then
      t = @head
      @head = nil
      return t.val
    end
    a = @head
    b = @head.next

    while b.next != nil do
      a = b
      b = b.next
    end

    a.next = nil
    return b.val
  end

  def print
    n = @head
    while n != nil do
      puts "Node(#{n.val})"
      n = n.next
    end
  end

end



ll = LinkedList.new([1, 2, 3, 4, 5])
ll.print
ll.push 6
ll.print
v = ll.pop
puts "the tail #{v}"
ll.print
