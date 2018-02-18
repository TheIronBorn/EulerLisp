; Solved 

(defn total (n sides (acc 0))
      (if (zero? n)
          acc
          (total (dec n)
                 sides
                 (+ acc (rand 1 sides)))))

(defn pete-wins? ()
  (let ([a (total 9 4)]
        [b (total 6 6)])
    (cond
      [(< a b) #f]
      [(> a b) #t]
      [else (pete-wins?)])))


(~>
  (combinations~ 9 (list 1 2 3 4))
  (map~ sum)
  (map~
    (fn (pete)
        (~> (combinations~ 6 (list 1 2 3 4 5 6))
            (map~ sum)
            (count~ &(> pete &1)))))
  sum~
  println)

