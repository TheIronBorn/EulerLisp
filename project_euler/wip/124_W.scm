; Solved: 28.1.18

(defn rad (n)
  (~> n
      prime-factors
     (reduce-product fst)))

(~> (range~ 1 100000)
    (map~ &(pair (rad &1) &1))
    collect
    sort
    (nth (dec 10000))
    rst
    (println "Solution: "))
