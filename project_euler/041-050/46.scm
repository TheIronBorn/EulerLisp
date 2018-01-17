; Solved 8.1

(defn goldbach? (n (sq 1))
  (cond
    (< n (* 2 sq sq)) #f
    (prime? (- n (* 2 sq sq))) #t
    else (goldbach? n (inc sq))))


(~>
  (step~ 9 2)
  (select~
    (fn (x) (not (or (prime? x)
                     (goldbach? x)))))
  (nth~ 0)
  (println "Solution: "))
