import tomllib
import datetime
import itertools
import sympy
import sympy.logic
import os
	
def write_char_array(file, name, char_array):
	file.write(f'pub(crate) const {name}: &[char] = &[\n')
	for ch in char_array:
		codepoint = ord(ch)
		file.write(f'    \'\\u{{{codepoint:04X}}}\', // {ch}\n')
	file.write(f'];\n\n')
	
def write_encoded_char_array(file, name, char_array):
	file.write(f'pub(crate) const {name}: &[[u8; 2]] = &[\n')
	for ch in char_array:
		utf8_bytes = ch.encode('utf-8')
		array = ', '.join(map(lambda b: f'0x{b:02X}', utf8_bytes))
		file.write(f'    [{array}], // {ch}\n')
	file.write(f'];\n\n')
	
def write_is_zalgo_char_fn(file, bitmasks, min_zalgo_char, max_zalgo_char):
	file.write('/// Check if a given char is a zalgo char.\n')
	file.write('pub(crate) fn is_zalgo_char(c: char) -> bool {\n')
	file.write('    let c = u32::from(c);\n\n')
	
	file.write(f'    if !({ord(min_zalgo_char)}..({ord(max_zalgo_char)} + 1)).contains(&c) {{\n')
	file.write('        return false;\n')
	file.write('    }\n\n')
	
	for i, (pos_bitmask, neg_bitmask) in enumerate(bitmasks):
		file.write(f'    let case_{i} = c & 0b{pos_bitmask} == 0b{pos_bitmask} && c & 0b{neg_bitmask} == 0;\n')
	file.write('\n')
		
	file.write('    ')
	for i in range(len(bitmasks)):
		file.write(f'case_{i}')
		if i + 1 < len(bitmasks):
			file.write(' || ')
	file.write('\n')
	file.write('}')
	
def int_to_bits(n):
	assert(n >= 0)
	
	bits = []
	while n > 0:
		bits.append(n & 1 != 0)
		n >>= 1
		
	return bits
	
def generate_zalgo_char_bitmasks(zalgo_char_data_up, zalgo_char_data_down, zalgo_char_data_mid):
	char_map = dict()
	for ch in itertools.chain(zalgo_char_data_up, zalgo_char_data_down, zalgo_char_data_mid):
		assert(ord(ch) < 0xFFFF)
		bits = int_to_bits(ord(ch))
		char_map[ch] = bits
		
	# max_char_len = len(max(char_map.values(), key=len))
	max_char_len = 32
	for ch in char_map:
		value = char_map[ch]
		while len(value) < max_char_len:
			value.append(0)
		char_map[ch] = value
		
	symbols = [sympy.Symbol(f'b{i}', bool=True) for i in range(max_char_len)]
	simplified_expr = sympy.logic.SOPform(symbols, char_map.values())

	# print(simplified_expr)
	assert(simplified_expr.func == sympy.logic.Or)
	def classify_bit(arg):
		is_not = arg.func == sympy.logic.Not
		bit = int((arg.name if not is_not else arg.args[0].name).lstrip('b'))
		return (bit, is_not)
		
	bitmasks = []
	for arg in simplified_expr.args:
		sorted_args = sorted(map(classify_bit, arg.args), key=lambda t: t[0])
		sorted_args_index = 0
		
		pos = []
		neg = []
		
		for i in range(max_char_len):
			if sorted_args_index < len(sorted_args) and i == sorted_args[sorted_args_index][0]:
				if sorted_args[sorted_args_index][1]:
					neg.append(1)
					pos.append(0)
				else:
					neg.append(0)
					pos.append(1)
				
				sorted_args_index += 1
			else:
				pos.append(0)
				neg.append(0)
		assert(sorted_args_index >= len(sorted_args))
		
		pos.reverse()
		neg.reverse()
		
		pos_bit_str = ''.join(map(str, pos))
		neg_bit_str = ''.join(map(str, neg))
		
		bitmasks.append((pos_bit_str, neg_bit_str))
		
	return bitmasks

def main():
	zalgo_char_data = None
	with open('zalgo-char-data.toml', 'rb') as file:
		zalgo_char_data = tomllib.load(file)
	zalgo_char_data_up = zalgo_char_data['up']
	zalgo_char_data_up.sort()
	zalgo_char_data_down = zalgo_char_data['down']
	zalgo_char_data_down.sort()
	zalgo_char_data_mid = zalgo_char_data['mid']
	zalgo_char_data_mid.sort()
	
	min_zalgo_char = min(itertools.chain(zalgo_char_data_up, zalgo_char_data_down, zalgo_char_data_mid), key=ord)
	max_zalgo_char = max(itertools.chain(zalgo_char_data_up, zalgo_char_data_down, zalgo_char_data_mid), key=ord)
	bitmasks = generate_zalgo_char_bitmasks(zalgo_char_data_up, zalgo_char_data_down, zalgo_char_data_mid)
	
	with open('src/chars.rs.part', 'w', encoding="utf-8") as rust_chars_file:
		rust_chars_file.write(f'// Generated on {datetime.datetime.now()} with `./scripts/build-char-tables.py`\n\n')
		
		rust_chars_file.write('/// Up zalgo chars\n')
		rust_chars_file.write('#[cfg(test)]\n')
		write_char_array(rust_chars_file, 'ZALGO_UP', zalgo_char_data_up)
		
		rust_chars_file.write('/// Encoded up zalgo chars\n')
		write_encoded_char_array(rust_chars_file, 'ZALGO_UP_ENCODED', zalgo_char_data_up)
		
		rust_chars_file.write('/// Down zalgo chars\n')
		rust_chars_file.write('#[cfg(test)]\n')
		write_char_array(rust_chars_file, 'ZALGO_DOWN', zalgo_char_data_down)
		
		rust_chars_file.write('/// Encoded down zalgo chars\n')
		write_encoded_char_array(rust_chars_file, 'ZALGO_DOWN_ENCODED', zalgo_char_data_down)
		
		rust_chars_file.write('/// Mid zalgo chars\n')
		rust_chars_file.write('#[cfg(test)]\n')
		write_char_array(rust_chars_file, 'ZALGO_MID', zalgo_char_data_mid)
		
		rust_chars_file.write('/// Encoded mid zalgo chars\n')
		write_encoded_char_array(rust_chars_file, 'ZALGO_MID_ENCODED', zalgo_char_data_mid)
		
		write_is_zalgo_char_fn(rust_chars_file, bitmasks, min_zalgo_char, max_zalgo_char)
		
	os.replace('src/chars.rs.part', 'src/chars.rs')
	
if __name__ == '__main__':
	main()