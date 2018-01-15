; Solved 14.1
; 
; Changes: Add `reduce-sum` and `reduce-product` to lisp stdlib

(defn split3 (data (cur 0) (acc0 '()) (acc1 '()) (acc2 '()))
      (if (nil? data)
        (list acc0 acc1 acc2)
        (split3
          (rst data)
          (% (inc cur) 3)
          (if (= cur 0) (cons (fst data) acc0) acc0)
          (if (= cur 1) (cons (fst data) acc1) acc1)
          (if (= cur 2) (cons (fst data) acc2) acc2))))

(defn count (e lst (acc 0))
      (cond
        (nil? lst) acc
        (= e (fst lst)) (count e (rst lst) (inc acc))
        else (count e (rst lst) acc)))

(defn frequency (lst)
      (~>
        (range 0 127)
        (map (fn (b) (count b lst)))))

(defn find-max-index (lst (index 0) (max-value 0) (max-index 0))
      (cond
        (nil? lst)
        max-index
        (> (fst lst) max-value)
        (find-max-index (rst lst) (inc index) (fst lst) index)
        else  
        (find-max-index (rst lst) (inc index) max-value max-index)))

(defn number->bits (n (remaining 8) (acc '()))
      (if (zero? remaining)
        acc
        (number->bits (/ n 2)
                      (dec remaining)
                      (cons (% n 2) acc))))

(defn bits->number (bits (acc 0))
      (if (nil? bits)
        acc
        (bits->number (rst bits)
                      (+ (fst bits) (* 2 acc)))))

(defn xor-bits (a b (acc '()))
      (if (nil? a)
        (reverse acc)
        (xor-bits (rst a) (rst b)
                  (cons 
                    (if (= 1 (+ (fst a) (fst b))) 1 0)
                    acc)))) 

(defn xor (a b)
      (bits->number
        (xor-bits
          (number->bits a)
          (number->bits b))))

(def numbers
     (~>
       (file-read "./project_euler/051-060/59.txt")
       lines
       fst
       (string-split ",")
       (map string->number)
       ))

(def splits (~> numbers split3))

; If the text contains spaces, 32 will be the most common value
(def key
     (map 
       (fn (split)
           (~> split frequency find-max-index (xor 32)))
       splits))

; Here key refers to just one byte of the full key
(defn sum-split-with-key (split key)
  (reduce-sum (fn (s) (xor s key))
              split))

(println "Solution: "
  (reduce-sum
    (fn (ks)
        (apply sum-split-with-key ks))
    (zip splits key)))
