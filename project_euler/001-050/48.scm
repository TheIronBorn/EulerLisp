; Solved 5.1.2018

(~> (range~ 1 1000)
    (map~ &(pow &1 &1))
    sum~
    number->digits
    reverse
    (take 10)
    digits->number
    solution)
