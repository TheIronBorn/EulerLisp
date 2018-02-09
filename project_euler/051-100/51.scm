; Solved: 20.1.2018

(defn replace (n i d)
  (let ([p1 (pow 10 i)])
       (+ (% n p1)
          (- n (% n (* 10 p1)))
          (* d p1))))

; It can't be the last digit,
; otherwise some of the numbers in the 8-number family
; would be even
(defn replacements (last len)
  (if (= len 1)
      (map list (range 1 (dec last)))
      (flatmap
        (fn (x)
            (map
              &(cons x &1)
              (replacements x (dec len))))
        (range len (dec last)))))

(defn all-replacements (digits)
  (flatmap
    &(replacements digits &1)
    (range 1 (dec digits))))

(def replacements5 (all-replacements 5))
(def replacements6 (all-replacements 6))
(def replacements7 (all-replacements 7))

(defn get-all-replacements (n)
  (case (inc (floor (log10 n)))
        [5 replacements5]
        [6 replacements6]
        [7 replacements7]))

(defn apply-replacement (n replacement digit)
  (if (nil? replacement)
      n
      (apply-replacement
        (replace n (fst replacement) digit)
        (rst replacement)
        digit)))

; The (> x n) is needed to avoid including
; 109 in the family of 111109
(def all-digits (range 0 9))
(defn creates-family? (n replacement)
    (let ([family (map &(apply-replacement n replacement &1) all-digits)])
      (if (= 8 (count &(and (prime? &1) (> &1 n)) family))
          (~>
            family
            (select prime?)
            sort
            fst
            solution)
          #f)))

(defn has-family? (n)
  (any?
    &(creates-family? n &1)
    (get-all-replacements n)))

(~>
  (step~ 56003 2)
  (select~ prime?)
  (select~ has-family?)
  first~)
