; Solved: ...

; Find a < m so that a^2 = a (mod m)
(defn find-idem_ (cur m)
  (if (= (% (* cur cur) m) cur)
      cur
      (find-idem_ (dec cur) m)))
(defn find-idem (m) (find-idem_ (dec m) m))

(~>
  (range~ 1 10000)
  (map~ find-idem)
  sum~
  (println "Solution: "))
