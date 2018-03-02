; Solved 6.1.2018

(defn number->digits2 (n)
  (defn inner (n acc)
      (if (= n 0)
          acc
          (inner (div n 2) (cons (% n 2) acc))))
  (inner n '()))

(defn build-plaindromes (n)
  (let* ([ds (number->digits n)]
         [rds (reverse ds)])
    (list
      (digits->number (append rds ds))
      (digits->number (append rds (rst ds))))))

(~> (range~ 1 1_000)
    (flatmap-list~ build-plaindromes)
    (select~ &(< &1 1_000_000))
    (select~ &(palindromic? (number->digits2 &1)))
    sum~
    solution)
