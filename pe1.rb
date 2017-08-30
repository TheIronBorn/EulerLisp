start = Time.now

100000.times do 
  count = 0

  (0..100).each do |i|
    if ((i % 15 != 0) && (i % 5 == 0 || i % 3 == 0))
      count += i
    end
  end
end

puts Time.now - start
