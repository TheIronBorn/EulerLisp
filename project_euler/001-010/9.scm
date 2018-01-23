; Solved: 20.12.17
; Changes:

(defn triplet? (a b c)
      (= (+ (square a) (square b))
         (square c)))

; Max possible:
; a = 332
; b = 333
; c = 335
; ====
; a = fixed
; b = (1000 - fixed - 1) / 2

; Just try to bruteforce it
(defn triplet (sum)
      (let* 
        ([a (rand 1 (div (dec sum) 3))]
         [b (rand a (div (- sum a) 2))]
         [c (- (- sum a) b)])
        (if (triplet? a b c)
            (list a b c)
            (triplet sum))))

(def result (triplet 1000))

(println "Triplet: " result)
(println "Solution: " (reduce * 1 result))
