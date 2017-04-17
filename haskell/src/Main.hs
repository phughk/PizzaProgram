data PizzaBase = Thin | Thick
data Topping = Tomatoe | Sweetcorn | Peppers | Olives
type Pizza = (PizzaBase, [Topping])

pizzaA :: Pizza
pizzaA = (Thin, [Tomatoe, Sweetcorn])

main = do
	inputdata <- getContents
	putStrLn $ inputdata
