; Solved: 28.1.18
; Proof: See notes

(defn r-max (a)
  (~> 
      (if (odd? a)
          (range~ 1 (* 2 a) 2)
          (range~ 1 a 2))
      (map~ &(% (* 2 &1 a) (* a a)))
      collect
      (reduce-max id 0)))

(~> (range~ 3 1000)
    (map~ &(max (r-max &1) 2))
    sum~
    (println "Solution: "))
