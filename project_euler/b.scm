(~>
  (range~ 1 1000000)
  (map~ inc)
  (select~ even?)
  (reduce~ + 0)
  println)


