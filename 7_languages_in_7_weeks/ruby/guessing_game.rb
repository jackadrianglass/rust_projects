number_to_guess = rand(100)
guessed = false

puts 'Please guess a number'

until guessed
  guess = gets.to_i
  if guess < number_to_guess then
    puts 'Too low'
  elsif guess > number_to_guess then
    puts 'Too high'
  else
    guessed = true
  end
end

puts 'Thanks for playing'
