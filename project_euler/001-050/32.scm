; Solved 6.1.2018

(def all-digits (range 1 9))

(defn pandigital? (a b c)
  (= (sort (flatmap number->digits (list a b c)))
     all-digits))

(def range1 (range 1 9))
(def range2 (range 12 98))
(def range3 (range 123 987))
(def range4 (range 1234 9876))

(defn find-pandigital_ (ra rb ra-backup acc)
  (cond
    [(nil? rb) acc]
    [(nil? ra) (find-pandigital_ ra-backup (rst rb) ra-backup acc)]
    [else (let* ([a (fst ra)]
                 [b (fst rb)]
                 [c (* a b)])
           (if (pandigital? a b c)
             (find-pandigital_ (rst ra) rb ra-backup (cons c acc))
             (find-pandigital_ (rst ra) rb ra-backup acc)))]))

(defn find-pandigital (ra rb)
  (find-pandigital_ ra rb ra '()))

; The product of a n-digit number and a m-digit number
; is either n+m-1 and n+m digits long,
; so the only possible combinations (with n <= m) are
; 1.  1 x 4 = 4
; 2.  2 x 3 = 4

(~>
  (find-pandigital range1 range4)
  (append (find-pandigital range2 range3))
  sort
  uniq
  list-sum
  solution)
