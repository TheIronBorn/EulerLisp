; Solved: 20.2.2018

; X_0 = 0
; X_1 = 0 | Y_0
; X_2 = 0 | Y_0 | Y_1
; ...
;
;   Pr[X_n] = 11111...
; = Pr[forall 0 <= i < 32 :
;    exists 0 <= j <= n : Y_n[j] = 1 (j-th bit)]
; = Pr[forall 0 <= i < 32 : not forall 0 <= j <= n : Y_n[j] = 0]
; = (1 - 2^{-n})^m
;
;   Pr[N > n] = Pr[X_n] != 1111... = 1 - (1 - 2^{-n})^m
;   Pr[N = n] = Pr[N > (n-1)] - Pr[N > n]
;             = (1 - 2^{-n})^m - (1 - 2^{-(n - 1))^m
;
;   Ex[N] = sum Pr[N = i] * i

(defn solve-n (n)
  {(pow {1 - (pow 0.5 n)} 32) - (pow {1 - (pow 0.5 (dec n))} 32)})

(defn solve ((cur 1) (acc 0.0))
  (let ([next (+ acc (* cur (solve-n cur)))])
    (if (= acc next)
        next
        (solve (inc cur) next))))

; TODO: Implement a way to round floats to n digits
(solution (solve))
