; Solved: 2.1.2018

; Given circle centered at (1,1)
; with radius 1,
; we want to find out where a line starting at (0, 0)
; and going to (n, 1) will hit it.
;
; For a fixed x, y = x/n.
;
;     x, y is on the circle
; <=> sqrt((x - 1)^2 + (y - 1)^2) = 1
; <=> x^2 - 2x + 1 + y^2 - 2y + 1 = 1
; <=> x^2 - 2x + 1 + (x/n)^2 - 2x/n + 1 = 1
; <=> x^2(1 + 1/n^2) - x2(1 + 1/n) + 1 = 0
;
; This quadratic equation can be solved easily
; and we only care about the smaller result.
;
; Next, we are interested in the area inside a circle,
; moving from the center to some 0 <= a <= 1.
;
; Area(a) = \int_0^{a} \sqrt{1 - t^2} dt
;         = 1/2 (\sqrt{1-a^2}a + sin^{-1}(a)) 
; 
; A = (x - 1) - Area(x - 1) `below-circle`
; B = x * y / 2 = x * x/n / 2
;
;-------1---------|
; ################|
;  ###############|
;   ##############1
;     ############|
;     /###########|
;   / |  #########|
; / B |  A  ######|
;--x--|-----a-----|

(def pi (get-pi))
(def full (/ (- 4 pi) 4))

(defn below-circle (x)
  (- x
    (/ (+ (*
            (sqrt (- 1 (square x)))
            x)
          (asin x))
       2)))

(defn solve (n)
  (let ([a (+ 1 (/ 1 (square n)))]
        [b (* -2 (+ 1 (/ 1 n)))]
        [c 1])
    (let ([x (fst (solve-quadratic a b c))])
      (+ (below-circle (- 1 x))
         (/ (* x (/ x n)) 2)))))

(defn percent (n) (/ (solve n) full))
(~>
  (step~ 4)
  (select~ &(< (percent &1) (/ 1 1000)))
  first~
  solution)
