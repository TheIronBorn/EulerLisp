; Solved 25.1.18

(defn count-between (d from to)
  (~>
    (range~ from to)
    (count~ &(= 1 (gcd &1 d)))))

(~>
  (range~ 4 12000)
  (map~ &(count-between &1 (ceil (/ &1 3)) (floor (/ &1 2))))
  sum~
  (println "Solution: "))

