; Solved 22.1.2018

(defn cube (x) (* x x x))

(defn count-other_ (cur target (cnt 0))
  (let ([next (~> cur cube number->digits sort)])
    (cond
      [(> (length next) (length target)) cnt]
      [(= next target) (count-other_ (inc cur) target (inc cnt))]
      [else (count-other_ (inc cur) target cnt)])))

(defn count-other (n)
  (count-other_ (inc n) (sort (number->digits (cube n)))))

(~>
  (step~ 1)
  (select~ &(= (count-other &1) 4))
  first~
  cube
  solution)
