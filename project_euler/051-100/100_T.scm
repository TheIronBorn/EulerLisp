; Solved: 20.2.2018

; (defn find-b (r)
;   (+ r 0.5 (sqrt (+ 0.25 (* 2 r r)))))

; (defn solve (r)
;   (let ([b (find-b r)])
;     (if (integral? b)
;       (println (cons r b)))
;     (solve (inc r))))
;
;                     r    b
;                   ==========
;                     6   15
;  6 + 2 * 15 - 1  = 35   85 = 15 + 70 = 15 + 2 * 35
; 35 + 2 * 85 - 1 = 204  493 = 85 + 408 = 2 * 204
;
; => r_new = r + 2b - 1
; => b_new = b + 2r_new
;
; TODO: This outputs the correct solution,
; but I'd like to prove why that works

(defn step (r b)
  (let* ([r-new (+ r (dec (* 2 b)))]
         [b-new (+ b r-new r-new)])
    (if {{r-new + b-new} > 1_000_000_000_000}
      b-new
      (step r-new b-new))))

(solution (step 6 15))
