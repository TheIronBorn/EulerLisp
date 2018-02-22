; Solved: 22.2.2018

; Generate all paths through a binary tree
; of depth 15,
; left (#t) means the player picked the blue disk (chance 1 / total),
; right (#f) means red (chance (total - 1) / total).
;
; Any path with more than n/2 1s wins

(def n 15)
(def half-n (div n 2))

(defn path-probability (path (total 2) (acc 1))
  (if (nil? path)
      acc
      (path-probability
        (rst path)
        (inc total)
        (if (fst path)
            (* acc 1)
            (* acc (dec total))))))

; Slightly faster than using `count`
(defn wins? (path (acc 0))
  (cond
    [(nil? path) #f]
    [(fst path)
     (if {acc >= half-n}
         #t
         (wins? (rst path) (inc acc)))]
    [else (wins? (rst path) acc)]))

(def denom (product~ (range~ 2 (inc n))))
(def num
  (~>
    (combinations~ n '(#f #t))
    (select~ wins?)
    (map~ path-probability)
    sum~))

(solution (div denom num))
