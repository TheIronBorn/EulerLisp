
(~>
  (range~ 1 1_000_000)
  (select~ even?)
  (map~ square)
  (map~ inc)
  (map~ inc)
  (select~ even?)
  (select~ even?)
  (select~ even?)
  (select~ even?)
  (select~ even?)
  (map~ square)
  sum~
  solution)
