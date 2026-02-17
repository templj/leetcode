class Solution:
    def decodeMessage(self, key: str, message: str) -> str:
        decode = {}
        count = 0
        for c in key:
            if c != ' ' and c not in decode:
                decode[c] = chr(count + ord('a'))
                count += 1
        res = []
        for c in message:
            if c == ' ':
                res.append(' ')
            else:
                res.append(decode[c])
        return ''.join(res)
