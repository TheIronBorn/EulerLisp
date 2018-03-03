; Solved: ...

; Find a < m so that a^2 = a (mod m)
(defn find-idem_ (a b m)
  (if (zero? (% (* a b) m))
      b
      (find-idem_ (dec a) a m)))
(defn find-idem (m) (find-idem_ {m - 2} {m - 1} m))

; 5000 in 8.21
(~>
  (range~ 1 5_000)
  (map~ find-idem)
  sum~
  (println "Solution: "))
