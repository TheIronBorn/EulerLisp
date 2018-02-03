; Solved: 3.2.2018

(defn tiles-used (outer inner)
  (- (square outer) (square inner)))

(defn find-inner-even (n outer (inner 2))
    (if (<= (tiles-used outer inner) n)
        (div (- outer inner) 2)
        (find-inner-even n outer (+ 2 inner))))

(defn find-inner-odd (n outer (inner 1))
  (if (<= (tiles-used outer inner) n)
      (div (- outer inner) 2)
      (find-inner-odd n outer (+ 2 inner))))

(defn make-even (n) (if (even? n) n (inc n)))
(defn make-odd (n) (if (odd? n) n (inc n)))

(defn min-inner (n outer)
  (if (> outer (sqrt n))
      (ceil (sqrt (- (square outer) n)))
      1))

(defn find-inner (n outer)
  (if (even? outer)
    (find-inner-even n outer (make-even (min-inner n outer)))
    (find-inner-odd n outer (make-odd (min-inner n outer)))))

(defn max-outer (n) (inc (div n 4)))

(def n 1_000_000)

(~>
  (range~ 3 (max-outer n))
  (map~ &(find-inner n &1))
  (sum~)
  solution)
