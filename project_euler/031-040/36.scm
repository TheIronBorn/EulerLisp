; Solved 6.1

(defn number->digits2 (n (acc '()))
      (if (= n 0)
          acc
          (number->digits2 (div n 2) (cons (% n 2) acc))))

(~>
  (range~ 1 1000000)
  (select~ 
    &(and
       (palindromic? (number->digits &1))
       (palindromic? (number->digits2 &1))))
  sum~
  (println "Solution: "))
