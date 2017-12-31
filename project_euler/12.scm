; Solved: 29.12.17
; Changes:
;  * add builtin `factors` method (although it seems like cheating)

(defn num-prime-factors (n)
  (product
    (map (fn (x) (inc (rst x)))
         (prime-factors n))))

(defn count-factors (n)
      (def r (isqrt n))
      (defn helper (cur acc)
            (if (> cur r)
              acc
              (if (divides? cur n)
                  (if (= cur (/ n cur))
                    (helper (inc cur) (+ acc 1))
                    (helper (inc cur) (+ acc 2)))
                  (helper (inc cur) acc))))
      (inc (helper 1 0)))

; (defn factors_ (n)
;   (defn helper (cur acc)
;         (if (> cur n)
;           acc
;           (if (divides? cur n)
;             (helper (inc cur) (inc acc))
;             (helper (inc cur) acc))))
;   (helper 2 1))

(defn factors_ (n)
  (def r (isqrt n))
  (defn helper (cur acc)
        (if (> cur r)
          acc
          (if (divides? cur n)
            (if (= cur r)
              (helper (inc cur) (+ acc 1))
              (helper (inc cur) (+ acc 2)))
            (helper (inc cur) acc))))
  (helper 2 2))

(defn solve (number n)
  (let ((npf (length (factors number))))
    (if (> npf 500)
      (println number)
      (solve (+ number n) (inc n)))))

(solve 1 2)
