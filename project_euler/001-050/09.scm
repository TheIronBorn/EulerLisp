; Solved: 20.12.2017

(def n 1000)

(defn triplet? (lst)
      (= (+ (square (fst lst))
            (square (frst lst)))
         (square (frrst lst))))

; Max possible:
; a = 332
; b = 333
; c = 335
; ====
; a = fixed
; b = (1000 - fixed - 1) / 2
;
; TODO: There are probably better formulas

(defn find-triplets (a)
  (~> (range-stream (inc a) (div (- n a) 2))
      (stream-map &(list a &1 (- n a &1)))
      (stream-select triplet?)))

(~> (range-stream 1 (div n 3))
    (stream-flatmap find-triplets)
    fst
    list-product
    solution)
