; Solved: 17.12.17
; Changes:
; * implement even?, odd?
; * implement & improve prime_factors method
; * implement map & reduce
; * fix parsing of '() as nil
; * implement pow

(def from 2)
(def to 20)

(defn merge (a1 a2)
      (cond
        (nil? a1) a2
        (nil? a2) a1
        else (let (k1 (ffst a1)
                      k2 (ffst a2)
                      v1 (rfst a1)
                      v2 (rfst a2))
               (cond
                 (= k1 k2) (cons
                             (cons k1 (max v1 v2))
                             (merge (rst a1) (rst a2)))
                 (< k1 k2) (cons
                             (cons k1 v1)
                             (merge (rst a1) a2))
                 else (cons
                        (cons k2 v2)
                        (merge a1 (rst a2)))))))

(def factors_ (map prime-factors (range from to)))
(def max-factors (reduce merge '() factors_))

(println "Max. factors: " max-factors)

(println
  "Solution: "
  (reduce (fn (factor acc)
              (* acc (pow (fst factor) (rst factor))))
          1 max-factors))
