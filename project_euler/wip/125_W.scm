(~>
  (range~ 2 100_000_000)
  (select~ &(palindromic? (number->digits &1)))
  (count~ (fn (x) #t))
  solution
  )
