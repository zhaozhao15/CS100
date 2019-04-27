import copy

class MatrixSyntaxError(Exception):
	pass

class Matrix:
	def __init__(self,s):
		if type(s) == list:
			self.data=s
			self.row=len(s)
			if self.row==0:
				self.column=0
			else:
				self.column=len(self.data[0])
		try:
			if type(s)==str:
				s=s.replace(' ','')
				if '+j' in s:
					s=s.replace('+j','+1j')
				if '-j' in s:
					s=s.replace('-j','-1j')
				if ',j' in s:
					s=s.replace(',j',',1j')
				if ';j' in s:
					s=s.replace(';j',';1j')
				if '[j' in s:
					s=s.replace('[j','[1j')
				if '0j' in s:
					s=s.replace('[j','[1j')
				if s=='[]':
					self.row=0
					self.column=0
					self.data=[]
				else:
					self.row=s.count(';')+1
					self.column=(s.count(','))//self.row+1
					s1=s.replace(';','],[')
					s1='['+s1+']'
					self.data=eval(s1)
				s2=s.strip('[')
				s2=s2.strip(']')
				try:
					s3=s2.replace(';',',')
					s3=s3.split(',')
				except:
					raise MatrixSyntaxError()
				s4=s2.split(';')
				n=len(s4)
				c=repr(s4[0])
				c=c.split(',')
				c=len(c)
				for i in range(n):
					a=repr(s4[i])
					a=a.split(',')
					b=len(a)
					if b != c:
						raise MatrixSyntaxError()
				if [] in self.data:
					raise MatrixSyntaxError()
		except:
			raise MatrixSyntaxError()			

	def __str__(self):
		if self.data==[]:
			return '[]'
		for i in range(self.row):
			for j in range(self.column):
				if self.data[i][j].imag==0:
					self.data[i][j]=self.data[i][j].real
				if abs(self.data[i][j])==0:
					self.data[i][j]=0
				if type(self.data[i][j])!=complex and self.data[i][j]==int( self.data[i][j]):
					self.data[i][j]=int(self.data[i][j])
		s=str(self.data)
		s=s.replace(' ','')
		s1=s.replace('],[',';')
		s1=s1[1:-1]
		s1=s1.replace('(','')
		s1=s1.replace(')','')
		s1=s1.replace('0+','')
		s1=s1.replace('0j','')
		return s1

	def __add__(self,x):
		if self.row != x.row or self.column != x.column:
			raise MatrixSyntaxError()
		s=[[self.data[i][j]+x.data[i][j] for j in range(self.column)] for i in range(self.row)]
		return Matrix(s)

	def __sub__(self,x):
		if self.row != x.row or self.column != x.column:
			raise MatrixSyntaxError()
		s=[[self.data[i][j]-x.data[i][j] for j in range(self.column)] for i in range(self.row)]
		return Matrix(s)

	def __mul__(self,x):
		if type(x)==Matrix:
			if self.column != x.row:
				raise MatrixSyntaxError()
			if self.column==x.row:
				s=[[0]*x.column for i in range(self.row)]
				for i in range(self.row):
					for j in range(x.column):
						for k in range(self.column):
							s[i][j]+=self.data[i][k]*x.data[k][j]
				return Matrix(s)
		else:
			s=[[self.data[i][j]*x for j in range(self.column)] for i in range(self.row)]
			return Matrix(s)

	def __neg__(self):
		s=self*-1
		return s

	def __truediv__(self,x):
		s=[[self.data[i][j]/x for j in range(self.column)] for i in range(self.row)]
		return Matrix(s)

	def __pow__(self,x) :
		if not self.isSquare():
			raise MatrixSyntaxError()
		s=1
		for i in range(x):
			s=self*s
		return s

	def __eq__(self,x) :
		if self.data == x.data:
			return True
		else:
			return False

	def transposition(self):
		data=[[self.data[j][i] for j in range(self.row)] for i in range(self.column)]
		return Matrix(data)

	def isSquare(self):
		if self.row==self.column:
			return True
		else:
			return False

	def isIdentity(self):
		if self.data==[]:
			return True
		if self.row==self.column:
			for i in range (self.row):
				if self.data[i][i]==1:
					self.data[i][i]=0
			sgn=0
			for i in range (self.row):
				for j in range (self.column):
					if self.data[i][j]!=0:
						sgn=1
						return False
			if sgn == 0:
				return True

	def __getitem__(self,x):
			x=str(x)
			x=x.replace(' ','')
			a=x
			if ',' in a and 'slice' not in a:
				x=copy.deepcopy(a)
				x=x.replace('(','')
				x=x.replace(')','')
				n=x.index(',')
				a=eval(x[0:n])
				b=eval(x[n+1:])
				return self.data[a][b]
			if 'slice' in a:
				x=copy.deepcopy(a)
				x=x.replace('slice','')
				x=x.replace('(','')
				x=x.replace(')','')
				x=x.split(',')
				start1=int(x[0])
				stop1=int(x[1])
				step1=int(x[2])
				start2=int(x[3])
				stop2=int(x[4])
				step2=int(x[5])
				if (stop1-start1)/step1==(stop1-start1)//step1:
					row=(stop1-start1)//step1
				else:
					row=(stop1-start1)//step1+1
				if (stop2-start2)/step2==(stop2-start2)//step2:
					column=(stop2-start2)//step2
				else:
					column=(stop2-start2)//step2+1
				y=[[self.data[start1+i*step1][start2+j*step2] for j in range (column) ]for i in range (row)]
				return Matrix(y)
			if ',' not in a :
				x=copy.deepcopy(a)
				x=eval(x)
				a=self.data[x]
				b=[]
				b.append(a)
				return Matrix(b)

	def __setitem__(self,i,j):
			a=str(i)
			a=a.replace(' ','')
			if ',' in a and 'slice' not in a:
				x=copy.deepcopy(a)
				x=x.replace('(','')
				x=x.replace(')','')
				n=x.index(',')
				c=eval(x[0:n])
				b=eval(x[n+1:])
				self.data[c][b]=j
			if 'slice' in a:
				x=copy.deepcopy(a)
				x=x.replace('slice','')
				x=x.replace('(','')
				x=x.replace(')','')
				x=x.split(',')
				start1=int(x[0])
				stop1=int(x[1])
				step1=int(x[2])
				start2=int(x[3])
				stop2=int(x[4])
				step2=int(x[5])
				if (stop1-start1)/step1==(stop1-start1)//step1:
					row=(stop1-start1)//step1
				else:
					row=(stop1-start1)//step1+1
				if (stop2-start2)/step2==(stop2-start2)//step2:
					column=(stop2-start2)//step2
				else:
					column=(stop2-start2)//step2+1
				if row!=j.row or column!= j.column:
					raise MatrixSyntaxError()
				for e in range (row):
					for f in range (column):
						self.data[start1+e*step1][start2+f*step2]=j.data[e][f]
			if ',' not in a :
				x=copy.deepcopy(a)
				if self.column!=len(j.data[0]):
					raise MatrixSyntaxError()
				x=eval(x)
				self.data[x]=j.data[0]

	def determinant(self):
		if self.row!=self.column:
			raise MatrixSyntaxError()
		if self.row==2:
			ans=self.data[0][0]*self.data[1][1]-self.data[0][1]*self.data[1][0]
			return ans
		if self.row > 2:
			ans=0
			for i in range(self.column):
				a=[[self.data[j][k] for k in range(self.column) if k !=i] for j in range(1,self.row)]
				newmat = Matrix(a)
				ans += self.data[0][i]*(-1)**i*newmat.determinant()
			return ans

	def inverse(self):
		if self.row!=self.column:
			raise MatrixSyntaxError()
		if self.row==2:
			a=self.determinant()
			self.data[0][0],self.data[1][1]=self.data[1][1],self.data[0][0]
			self.data[1][1],self.data[0][0]=self.data[1][1]/a,self.data[0][0]/a
			self.data[0][1],self.data[1][0]=-self.data[0][1]/a,-self.data[1][0]/a
			a=self.data
			b=Matrix(a)
			return b
		if self.row > 2 :
			M=[[0]*self.column for i in range(self.row)]
			for i in range(self.column):
				for j in range (self.column):
					a=[[self.data[k][l] for l in range(self.column) if l !=j] for k in range(self.row)if k !=i]
					b=Matrix(a)
					c=b.determinant()
					c=(-1)**(i+j)*c
					M[i][j]=c
			T=Matrix(M)
			T=T.transposition()
			d=self.determinant()
			if d == 0 :
				raise MatrixSyntaxError()
			ans=T/d
			return ans
