; Solved 8.1

(defn goldbach? (n (sq 1))
  (cond
    (< n (* 2 sq sq)) #f
    (prime? (- n (* 2 sq sq))) #t
    else (goldbach? n (inc sq))))


(~>
  (step~ 9 2)
  (select~ &(not (or (prime? &1) (goldbach? &1))))
  (nth~ 0)
  (println "Solution: "))
