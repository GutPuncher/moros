(defn eq? (a b)
  (eq a b))

(defn atom? (x)
  (atom x))

(defn string? (x)
  (eq? (type x) "string"))

(defn boolean? (x)
  (eq? (type x) "boolean"))

(defn symbol? (x)
  (eq? (type x) "symbol"))

(defn number? (x)
  (eq? (type x) "number"))

(defn list? (x)
  (eq? (type x) "list"))

(defn function? (x)
  (eq? (type x) "function"))

(defn lambda? (x)
  (eq? (type x) "lambda"))

(def null '())

(defn null? (x)
  (eq? x null))

(defn first (lst)
  (car lst))

(defn rest (lst)
  (cdr lst))

(defn second (lst)
  (first (rest lst)))

(defn third (lst)
  (second (rest lst)))

(defn println (exp)
  (do (print exp) (print "\n")))
