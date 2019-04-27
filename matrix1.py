def IsStandard(A):
	if A[0]==1:
		return True
	else:
		return False

def Str2Mat(s):
	if s=='[]':
		L=[]
		L.insert(0,1)
		L.insert(1,0)
		L.insert(2,0)
	elif s[0]=='[':
		s1=s.strip('[')
		s1=s1.strip(']')
		if ';' not in s:
			row=1
		else:
			n=s1.index(';')
			row=s1.count(';')+1
		column=(s1.count(','))//row+1
		a='],['
		s=s.replace(';',a)
		b='['
		c=']'
		s=b+s+c
		L=eval(s)
		L.insert(0,1)
		L.insert(1,row)
		L.insert(2,column)
	elif s=='0-0{}':
		L=[]
		L.insert(0,0)
		L.insert(1,0)
		L.insert(2,0)
	else:
		a=s.index('-')
		b=s.index('{')
		c=s.index('}')
		row=int(s[0:a])
		column=int(s[a+1:b])
		s=s[b+1:c]
		L=[]
		for j in range(row):
			L1=[]
			for i in range(column):
				L1.append(0)
			L.append(L1)
		s=s.replace('(', '')
		s=s.replace(')', '')
		s1=s.split(',')
		n=(len(s1))//3
		for k in range(n):
			s2=s1[3*k:3*k+3]
			x=int(s2[0])
			y=int(s2[1])
			z=eval(s2[2])
			L[x-1][y-1]=z
		L.insert(0,0)
		L.insert(1,row)
		L.insert(2,column)
	return L

def Mat2StrStandard(AA):
	row=AA[1]
	column=AA[2]
	A=AA[3:]
	if row==0:
		return '[]'
	else:
		s1='['
		for i in range(row):
			s=''
			for j in range(column):
				s=s+str(A[i][j])+','
			s=s.rstrip(',')
			s=s+';'
			s1=s1+s
		s1=s1.rstrip(';')
		s1=s1+']'
		s1=s1.replace('(','')
		s1=s1.replace(')','')
		return s1

def Mat2StrSparse(AA):
	row=AA[1]
	column=AA[2]
	A=AA[3:]
	if row==0:
		return '0-0{}'
	else:
		s=str(row)+'-'+str(column)+'{'
		for i in range(row):
			for j in range(column):
				if A[i][j]!=0:
					a=str(i+1)
					b=str(j+1)
					c=str(A[i][j])
					c=c.replace(')','')
					c=c.replace('(','')
					s1='('+a+','+b+','+c+')'+','
					s=s+s1
		s=s.rstrip(',')
		s=s+'}'
		return s
		
def Standard2Sparse(A):
	A[0]=0
	return A

def Sparse2Standard(A):
	A[0]=1
	return A

def MatAdd(AA, BB):
	D=AA[0:3]
	row=AA[1]
	column=AA[2]
	A=AA[3:]
	B=BB[3:]
	C=[[A[i][j]+B[i][j] for j in range(column)] for i in range(row)]
	E=D+C
	return E

def MatSub(AA, BB):
	D=AA[0:3]
	row=AA[1]
	column=AA[2]
	A=AA[3:]
	B=BB[3:]
	C=[[A[i][j]-B[i][j] for j in range(column)] for i in range(row)]
	E=D+C
	return E

def MatScalarMul(AA, c):
	D=AA[0:3]
	row=AA[1]
	column=AA[2]
	A=AA[3:]
	C=[[A[i][j]*c for j in range(column)] for i in range(row)]
	E=D+C
	return E

def MatTransposition(AA):	
	row=AA[1]
	column=AA[2]
	D=[]
	D.append(AA[0])
	D.append(column)
	D.append(row)
	A=AA[3:]
	C=[[A[j][i] for j in range(row)] for i in range(column)]
	E=D+C
	return E

def MatEq(AA, BB):
	A=AA[1:]
	B=BB[1:]
	if A==B:
		return True
	else:
		return False
		