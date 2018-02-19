; Solved: 20.2.2018


; streak(n) >= k
; <=> forall 1 < i <= k : i | n + (i - 1)
; <=> forall 1 < i <= k : i | (n - 1) + i
; <=> forall 1 < i <= k : i | (n - 1)
; <=> lcm(1, ..., k) | (n - 1)
;
; streak(n) = k
; <=> streak(n) >= k & !(streak(n) >= (k + 1))
;
; number of n < N with streak(n) = k:
; ((n - 2) / lcm(1, ..., k)) - ((n - 2) / lcm(1, ..., k + 1))
;     \
;      \ because n < n and we were working with n - 1 before

(def lcms
  (~>
    (range~ 1 32)
    (accumulate~
      (fn (acc n)
        (lcm acc n)
        )
      1)
    collect
    list->vector))

(defn solve (i)
  (let ([to (- (pow 4 i) 2)])
    (-
     (div to (vector-ref lcms (dec i)))
     (div to (vector-ref lcms i)))))

(~>
  (range~ 1 31)
  (map~ solve)
  sum~
  solution)
