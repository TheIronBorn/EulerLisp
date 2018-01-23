; Solved 22.1.18

(defn closest (d (cur 1) (last '()))
  (cond
    (!= 1 (gcd cur d)) (closest d (inc cur) last)
    (> (/ cur d) (/ 3 7)) last
    else (closest d (inc cur) (/ cur d))))


(def target (/ 3 7))
(defn closest_ (d cur (last '()))
  (let (this (/ cur d))
    (cond
      (> this target) last
      else (closest d (inc cur) this))))

(defn solve (d max-frac)
  (println "d = " d)
  (cond
    (> d 1000000) max-frac
    (= d 7) (solve (inc d) max-frac)
    else (solve (inc d) (max max-frac (closest d)))))
; (defn solve (d max-frac)
;   (println "d = " d)
;   (cond
;     (> d 1000000) max-frac
;     (= d 7) (solve (inc d) max-frac)
;     else (solve (inc d) (max max-frac (closest d
;                                                (* 2 (div d 7))
;                                                )))))

(defn find-prime ((cur 900001) (last 3))
  (cond
    (> cur 1000000) last
    (prime? cur) (find-prime (+ cur 2) cur)
    else (find-prime (+ cur 2) last)))

; TODO: This is a bit hacky
; and might not work in every case
(~>
  (solve (find-prime) (/ 1 1000000))
  numerator
  (println "Solution: "))

; (~>
;   (solve 3 (/ 1 1000000))
;   numerator
;   (println "Solution: "))
